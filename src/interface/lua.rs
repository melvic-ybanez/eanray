use crate::core::camera::Image;
use crate::core::color::ColorKind;
use crate::core::materials::{refractive_index, Dielectric, Lambertian, Metal};
use crate::core::math::vector::{PointKind, VecKind};
use crate::core::math::{Point, Real, Vec3D, VecLike};
use crate::core::shapes::Sphere;
use crate::core::{Camera, Color, Hittable, HittableList, Material};
use crate::settings;
use crate::settings::Config;
use mlua::{Function, Lua, LuaSerdeExt, Result, Table, Value};
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

fn new_table(lua: &Lua, function: Result<Function>) -> Result<Table> {
    let table = lua.create_table()?;
    table.set("new", function?)?;
    Ok(table)
}

fn new_vec_like_table<K>(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, x, y, z): (Table, Real, Real, Real)| {
            let vec3d = VecLike::<K>::new(x, y, z);
            Ok(lua.to_value(&vec3d))
        }),
    )
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

    table.set(
        "new_glass",
        lua.create_function(|lua, _: Table| {
            let dielectric = Material::Dielectric(Dielectric::new(refractive_index::GLASS));
            Ok(lua.to_value(&dielectric))
        })?,
    )?;

    Ok(table)
}

pub fn set_engine(lua: &Lua) -> Result<()> {
    let engine = lua.create_table()?;

    engine.set("Vec", new_vec_like_table::<VecKind>(lua)?)?;
    engine.set("Point", new_vec_like_table::<PointKind>(lua)?)?;
    engine.set("Color", new_vec_like_table::<ColorKind>(lua)?)?;

    let materials = lua.create_table()?;
    materials.set("Lambertian", new_lambertian_table(lua)?)?;
    materials.set("Metal", new_metal_table(lua)?)?;
    materials.set("Dielectric", new_dielectric_table(lua)?)?;
    engine.set("materials", materials)?;

    let shapes = lua.create_table()?;
    shapes.set("Sphere", new_sphere_table(lua)?)?;
    engine.set("shapes", shapes)?;

    lua.globals().set("engine", engine)?;

    Ok(())
}
