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
    AnyUserData, Function, Lua, LuaSerdeExt, MetaMethod, Result, Table, UserData, UserDataMethods,
    Value,
};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneSchema<'a> {
    camera: CameraSchema,
    objects: Vec<Hittable<'a>>,
}

impl<'a> SceneSchema<'a> {
    fn new(camera: CameraSchema, objects: Vec<Hittable<'a>>) -> Self {
        Self { camera, objects }
    }

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
    fn new(aspect_ratio: Real, image_width: u32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel: None,
            antialiasing: None,
            max_depth: None,
            field_of_view: None,
            look_from: None,
            look_at: None,
            defocus_angle: None,
            focus_distance: None,
            vup: None,
        }
    }

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
        methods.add_meta_method(MetaMethod::Add, |lua, this, other: Value| {
            let other_vec3: Vec3D = lua.from_value(other)?;
            Ok(this + other_vec3)
        });
    }
}

fn add_addable_vec_methods<K: 'static + CanAdd + Clone, M: UserDataMethods<VecLike<K>>>(
    methods: &mut M,
) where
    VecLike<K>: UserData,
{
    methods.add_meta_method(MetaMethod::Mul, |_, this, rhs| match rhs {
        Value::Integer(scalar) => Ok(this * scalar as Real),
        Value::Number(scalar) => Ok(this * scalar),
        Value::UserData(userdata) => {
            let other_vec_like: VecLike<K> = userdata.borrow::<VecLike<K>>()?.clone();
            Ok(this * other_vec_like)
        }
        _ => Err(mlua::Error::RuntimeError("Invalid RHS".into())),
    });
    methods.add_meta_method(MetaMethod::Add, |lua, this, other: Value| {
        let other_vec_like: VecLike<K> = lua.from_value(other)?;
        Ok(this + other_vec_like)
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
    table.set(
        "random_range",
        lua.create_function(|lua, (min, max): (Real, Real)| {
            let vec_like: VecLike<K> = VecLike::<K>::random_range(min, max);
            Ok(lua.create_ser_userdata(vec_like))
        })?,
    )?;

    Ok(table)
}

fn new_sphere_table(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;
    table.set(
        "stationary",
        lua.create_function(
            |lua, (_, center, radius, material): (Table, AnyUserData, Real, Value)| {
                let center: Point = center.borrow::<Point>()?.clone();
                let material: Material = lua.from_value(material)?;
                let sphere = Hittable::Sphere(Sphere::stationary(center, radius, material));
                Ok(lua.to_value(&sphere))
            },
        )?,
    )?;
    table.set(
        "moving",
        lua.create_function(
            |lua,
             (_, center1, center2, radius, material): (
                Table,
                AnyUserData,
                AnyUserData,
                Real,
                Value,
            )| {
                let center1: Point = center1.borrow::<Point>()?.clone();
                let center2: Point = center2.borrow::<Point>()?.clone();
                let material: Material = lua.from_value(material)?;
                let sphere = Hittable::Sphere(Sphere::moving(center1, center2, radius, material));
                Ok(lua.to_value(&sphere))
            },
        )?,
    )?;
    Ok(table)
}

fn new_lambertian_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, albedo): (Table, AnyUserData)| {
            let albedo: Color = albedo.borrow::<Color>()?.clone();
            let lambertian = Material::Lambertian(Lambertian::new(albedo));
            Ok(lua.to_value(&lambertian))
        }),
    )
}

fn new_metal_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, albedo, fuzz): (Table, AnyUserData, Real)| {
            let albedo: Color = albedo.borrow::<Color>()?.clone();
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

fn new_camera_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, image_width, aspect_ratio): (Table, u32, Real)| {
            let camera = CameraSchema::new(aspect_ratio, image_width);
            Ok(lua.to_value(&camera))
        }),
    )
}

fn new_object_list_table(lua: &Lua) -> Result<Table> {
    let objects = new_table(lua, lua.create_function(|_, this: Table| Ok(this)))?;
    objects.set(
        "add",
        lua.create_function(|lua, (this, object): (Table, Value)| {
            // Let's do a round-trip conversion for now to validate the structure.
            // This may not be the cleanest solution.
            let hittable: Hittable = lua.from_value(object)?;

            let next_index = this.raw_len() + 1;
            this.set(next_index, lua.to_value(&hittable)?)
        })?,
    )?;
    Ok(objects)
}

fn new_scene_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, camera, objects): (Table, Value, Value)| {
            let camera: CameraSchema = lua.from_value(camera)?;
            let objects: Vec<Hittable> = lua.from_value(objects)?;
            let scene: SceneSchema = SceneSchema::new(camera, objects);
            Ok(lua.to_value(&scene))
        }),
    )
}

pub fn set_engine(lua: &Lua) -> Result<()> {
    let engine = lua.create_table()?;

    engine.set("math", new_math_table(lua)?)?;
    engine.set("Color", new_vec_like_table::<ColorKind>(lua)?)?;
    engine.set("materials", new_materials_table(lua)?)?;
    engine.set("shapes", new_shapes_table(lua)?)?;
    engine.set("Camera", new_camera_table(lua)?)?;
    engine.set("ObjectList", new_object_list_table(lua)?)?;
    engine.set("Scene", new_scene_table(lua)?)?;

    lua.globals().set("engine", engine)?;

    Ok(())
}
