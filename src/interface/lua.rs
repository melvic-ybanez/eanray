use crate::core::camera::Image;
use crate::core::color::ColorKind;
use crate::core::materials::{refractive_index, Dielectric, Lambertian, Metal};
use crate::core::math::vector::{CanAdd, PointKind, VecKind};
use crate::core::math::{Point, Real, Vec3D, VecLike};
use crate::core::shapes::Sphere;
use crate::core::{math, Camera, Color, Hittable, HittableList, Material};
use crate::settings;
use crate::settings::Config;
use mlua::{
    Function, Lua, LuaSerdeExt, MetaMethod, Result, Table, UserData, UserDataMethods, Value,
};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneSchema {
    camera: CameraSchema,
    objects: Vec<Hittable>,
}

impl SceneSchema {
    pub fn render(&self, config: &'static Config) -> io::Result<()> {
        let camera = self.camera.build(config);
        camera.render(
            &Hittable::List(HittableList::from_vec(self.objects.clone())),
            config,
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CameraSchema {
    aspect_ratio: Real,
    image_width: u32,
    samples_per_pixel: Option<u32>,
    antialiasing: Option<bool>,
    max_depth: Option<u32>,
    field_of_view: Option<Real>,
    look_from: Option<Point>,
    look_at: Option<Point>,
    defocus_angle: Option<Real>,
    focus_distance: Option<Real>,
    vup: Option<Vec3D>,
}

impl CameraSchema {
    fn build(&self, config: &'static Config) -> Camera {
        let defaults = config.app().scene().camera().defaults();

        fn build_vec_like<K: Clone>(
            vec_like: &Option<VecLike<K>>,
            default: settings::Point,
        ) -> VecLike<K> {
            vec_like
                .clone()
                .unwrap_or(VecLike::<K>::new(default[0], default[1], default[2]))
        }

        Camera::builder(config)
            .image(Image::new(self.image_width, self.aspect_ratio))
            .antialiasing(self.antialiasing.unwrap_or(defaults.antialiasing()))
            .samples_per_pixel(
                self.samples_per_pixel
                    .unwrap_or(defaults.samples_per_pixel()),
            )
            .max_depth(self.max_depth.unwrap_or(defaults.max_depth()))
            .field_of_view(self.field_of_view.unwrap_or(defaults.field_of_view()))
            .look_from(build_vec_like(&self.look_from, defaults.look_from()))
            .look_at(build_vec_like(&self.look_at, defaults.look_at()))
            .defocus_angle(self.defocus_angle.unwrap_or(defaults.defocus_angle()))
            .focus_distance(self.focus_distance.unwrap_or(defaults.focus_distance()))
            .vup(build_vec_like(&self.vup, defaults.vup()))
            .build()
    }
}

impl UserData for Vec3D {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_addable_vec_methods(methods);
        methods.add_method("length", |_, this, ()| Ok(this.length()))
    }
}

impl UserData for Color {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_addable_vec_methods(methods);
    }
}

impl UserData for Point {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Sub, |lua, this, other: Value| {
            let point: Point = lua.from_value(other)?;
            Ok(this - point)
        });
    }
}

fn add_addable_vec_methods<K: 'static + CanAdd + Clone, M: UserDataMethods<VecLike<K>>>(
    methods: &mut M,
) where
    VecLike<K>: UserData,
{
    methods.add_meta_method(MetaMethod::Mul, |lua, this, rhs| match rhs {
        Value::Integer(scalar) => Ok(this * scalar as Real),
        Value::Number(scalar) => Ok(this * scalar),
        Value::UserData(userdata) => {
            let other_color: VecLike<K> = userdata.borrow::<VecLike<K>>()?.clone();
            Ok(this * other_color)
        }
        _ => Err(mlua::Error::RuntimeError("Invalid RHS".into())),
    });
    methods.add_meta_method(MetaMethod::Add, |lua, this, other: Value| {
        let other_color: VecLike<K> = lua.from_value(other)?;
        Ok(this + other_color)
    });
}

fn new_table(lua: &Lua, function: Result<Function>) -> Result<Table> {
    let table = lua.create_table()?;
    table.set("new", function?)?;
    Ok(table)
}

fn new_vec_like_table<K: 'static>(lua: &Lua) -> Result<Table>
where
    VecLike<K>: UserData,
{
    let table = new_table(
        lua,
        lua.create_function(|lua, (_, x, y, z): (Table, Real, Real, Real)| {
            let vec_like: VecLike<K> = VecLike::<K>::new(x, y, z);
            Ok(lua.create_ser_userdata(vec_like))
        }),
    )?;

    table.set("ZERO", lua.to_value(&VecLike::<K>::zero())?)?;
    table.set(
        "random",
        lua.create_function(|lua, ()| {
            let vec_like: VecLike<K> = VecLike::<K>::random();
            Ok(lua.create_ser_userdata(vec_like))
        })?,
    )?;

    Ok(table)
}

fn new_sphere_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(
            |lua, (_, center, radius, material): (Table, Value, Real, Value)| {
                let center: Point = lua.from_value(center)?;
                let material: Material = lua.from_value(material)?;
                let sphere = Hittable::Sphere(Sphere::new(center, radius, material));
                Ok(lua.to_value(&sphere))
            },
        ),
    )
}

fn new_lambertian_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, albedo): (Table, Value)| {
            let albedo: Color = lua.from_value(albedo)?;
            let lambertian = Material::Lambertian(Lambertian::new(albedo));
            Ok(lua.to_value(&lambertian))
        }),
    )
}

fn new_metal_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, albedo, fuzz): (Table, Value, Real)| {
            let albedo: Color = lua.from_value(albedo)?;
            let metal = Material::Metal(Metal::new(albedo, fuzz));
            Ok(lua.to_value(&metal))
        }),
    )
}

fn new_dielectric_table(lua: &Lua) -> Result<Table> {
    let table = new_table(
        lua,
        lua.create_function(|lua, (_, refraction_index): (Table, Real)| {
            let dielectric = Material::Dielectric(Dielectric::new(refraction_index));
            Ok(lua.to_value(&dielectric))
        }),
    )?;

    let refractive_index = lua.create_table()?;
    refractive_index.set("GLASS", refractive_index::GLASS)?;
    refractive_index.set("VACUUM", refractive_index::VACUUM)?;
    refractive_index.set("AIR", refractive_index::AIR)?;
    refractive_index.set("WATER", refractive_index::WATER)?;
    refractive_index.set("DIAMOND", refractive_index::DIAMOND)?;

    table.set("RefractiveIndex", refractive_index)?;

    Ok(table)
}

fn new_math_table(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;

    table.set("Vec", new_vec_like_table::<VecKind>(lua)?)?;

    table.set("Point", new_vec_like_table::<PointKind>(lua)?)?;
    table.set("random", lua.create_function(|_, ()| Ok(math::random()))?)?;
    table.set(
        "random_range",
        lua.create_function(|_, (min, max)| Ok(math::random_range(min, max)))?,
    )?;
    Ok(table)
}

fn new_materials_table(lua: &Lua) -> Result<Table> {
    let materials = lua.create_table()?;
    materials.set("Lambertian", new_lambertian_table(lua)?)?;
    materials.set("Metal", new_metal_table(lua)?)?;
    materials.set("Dielectric", new_dielectric_table(lua)?)?;
    Ok(materials)
}

fn new_shapes_table(lua: &Lua) -> Result<Table> {
    let shapes = lua.create_table()?;
    shapes.set("Sphere", new_sphere_table(lua)?)?;
    Ok(shapes)
}

pub fn set_engine(lua: &Lua) -> Result<()> {
    let engine = lua.create_table()?;

    engine.set("math", new_math_table(lua)?)?;
    engine.set("Color", new_vec_like_table::<ColorKind>(lua)?)?;
    engine.set("materials", new_materials_table(lua)?)?;
    engine.set("shapes", new_shapes_table(lua)?)?;

    lua.globals().set("engine", engine)?;

    Ok(())
}
