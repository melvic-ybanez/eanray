use crate::bindings::schemas::{CameraSchema, SceneSchema};
use crate::core::camera::Background;
use crate::core::color::ColorKind;
use crate::core::hit::ConstantMedium;
use crate::core::materials::{refractive_index, Dielectric, DiffuseLight, Lambertian, Metal};
use crate::core::math::vector::{PointKind, VecKind};
use crate::core::math::{Point, Real, Vec3D, VecLike};
use crate::core::shapes::planar::{Planar, Quad, Triangle};
use crate::core::shapes::{planar, Sphere};
use crate::core::textures::{Checker, ImageTexture, NoiseTexture, Texture};
use crate::core::transforms::{Rotate, RotateKind, Translate};
use crate::core::Hittable::BVH;
use crate::core::{bvh, math, Color, Hittable, HittableList, Material};
use mlua::{AnyUserData, Function, Lua, LuaSerdeExt, Result, Table, UserData, Value};
use std::sync::Arc;

macro_rules! from_user_data {
    ($name: ident, $t: ty) => {
        $name.borrow::<$t>()?.clone()
    };
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
    let new_function = lua.create_function(
        |lua, (_, center, radius, material): (Table, AnyUserData, Real, Value)| {
            let center = from_user_data!(center, Point);
            let material: Material = lua.from_value(material)?;
            let sphere = Hittable::Sphere(Sphere::stationary(center, radius, material));
            Ok(lua.to_value(&sphere))
        },
    );
    let table = new_table(lua, new_function.clone())?;
    table.set("stationary", new_function?)?;
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
                let center1 = from_user_data!(center1, Point);
                let center2 = from_user_data!(center2, Point);
                let material: Material = lua.from_value(material)?;
                let sphere = Hittable::Sphere(Sphere::moving(center1, center2, radius, material));
                Ok(lua.to_value(&sphere))
            },
        )?,
    )?;
    Ok(table)
}

fn new_planar_table(lua: &Lua, kind: planar::Kind) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(
            move |lua, (_, q, u, v, mat): (Table, AnyUserData, AnyUserData, AnyUserData, Value)| {
                let q = from_user_data!(q, Point);
                let u = from_user_data!(u, Vec3D);
                let v = from_user_data!(v, Vec3D);
                let mat: Material = lua.from_value(mat)?;
                let quad = Hittable::Planar(Planar::new(q, u, v, mat, kind.clone()));
                Ok(lua.to_value(&quad))
            },
        ),
    )
}

fn new_disk_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(
            move |lua,
                  (_, q, u, v, radius, mat): (
                Table,
                AnyUserData,
                AnyUserData,
                AnyUserData,
                Real,
                Value,
            )| {
                let q = from_user_data!(q, Point);
                let u = from_user_data!(u, Vec3D);
                let v = from_user_data!(v, Vec3D);
                let mat: Material = lua.from_value(mat)?;
                let quad = Hittable::Planar(Planar::disk(q, u, v, radius, mat));
                Ok(lua.to_value(&quad))
            },
        ),
    )
}

fn new_box_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(
            |lua, (_, a, b, mat): (Table, AnyUserData, AnyUserData, Value)| {
                let a = from_user_data!(a, Point);
                let b = from_user_data!(b, Point);
                let mat = lua.from_value(mat)?;
                let hl_box = Hittable::List(HittableList::make_box(a, b, mat));
                Ok(lua.to_value(&hl_box))
            },
        ),
    )
}

fn new_lambertian_table(lua: &Lua) -> Result<Table> {
    // TODO: should be `from_texture` instead of `new`
    let table = new_table(
        lua,
        lua.create_function(|lua, (_, texture): (Table, Value)| {
            let texture: Texture = lua.from_value(texture)?;
            let lambertian = Material::Lambertian(Lambertian::new(texture));
            Ok(lua.to_value(&lambertian))
        }),
    )?;

    table.set(
        "from_albedo",
        lua.create_function(|lua, (_, albedo): (Table, AnyUserData)| {
            let albedo: Color = from_user_data!(albedo, Color);
            let lambertian = Material::Lambertian(Lambertian::from_albedo(albedo));
            Ok(lua.to_value(&lambertian))
        })?,
    )?;

    Ok(table)
}

fn new_metal_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, albedo, fuzz): (Table, AnyUserData, Real)| {
            let albedo: Color = from_user_data!(albedo, Color);
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

fn new_diffuse_light_table(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "from_emission",
        lua.create_function(|lua, (_, emission_color): (Table, AnyUserData)| {
            let emission_color: Color = from_user_data!(emission_color, Color);
            let diffuse_light = Material::DiffuseLight(DiffuseLight::from_emission(emission_color));
            Ok(lua.to_value(&diffuse_light))
        })?,
    )?;
    table.set(
        "from_texture",
        lua.create_function(|lua, (_, texture): (Table, Value)| {
            let texture: Texture = lua.from_value(texture)?;
            let diffuse_light = Material::DiffuseLight(DiffuseLight::from_texture(texture));
            Ok(lua.to_value(&diffuse_light))
        })?,
    )?;

    Ok(table)
}

fn new_math_table(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;

    table.set("Vec", new_vec_like_table::<VecKind>(lua)?)?;

    table.set("Point", new_vec_like_table::<PointKind>(lua)?)?;
    table.set(
        "random",
        lua.create_function(|_, ()| Ok(math::random_real()))?,
    )?;
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
    materials.set("DiffuseLight", new_diffuse_light_table(lua)?)?;
    Ok(materials)
}

fn new_checker_table(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;
    table.set(
        "from_colors",
        lua.create_function(
            |lua, (_, scale, c1, c2): (Table, Real, AnyUserData, AnyUserData)| {
                let c1 = from_user_data!(c1, Color);
                let c2 = from_user_data!(c2, Color);
                let checker = Texture::Checker(Checker::from_colors(scale, c1, c2));
                Ok(lua.to_value(&checker))
            },
        )?,
    )?;

    Ok(table)
}

fn new_image_texture_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, filepath): (Table, String)| {
            let image_texture = Texture::Image(ImageTexture::from_path_unsafe(filepath.as_str()));
            Ok(lua.to_value(&image_texture))
        }),
    )
}

fn new_noise_texture_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, scale, base_color): (Table, f64, AnyUserData)| {
            let base_color: Color = from_user_data!(base_color, Color);
            let noise_texture = Texture::Noise(NoiseTexture::new(scale, base_color));
            Ok(lua.to_value(&noise_texture))
        }),
    )
}

fn new_textures_table(lua: &Lua) -> Result<Table> {
    let textures = lua.create_table()?;
    textures.set("Checker", new_checker_table(lua)?)?;
    textures.set("Image", new_image_texture_table(lua)?)?;
    textures.set("Noise", new_noise_texture_table(lua)?)?;
    Ok(textures)
}

fn new_shapes_table(lua: &Lua) -> Result<Table> {
    let shapes = lua.create_table()?;
    shapes.set("Sphere", new_sphere_table(lua)?)?;
    shapes.set("Quad", new_planar_table(lua, planar::Kind::Quad(Quad))?)?;
    shapes.set(
        "Triangle",
        new_planar_table(lua, planar::Kind::Triangle(Triangle))?,
    )?;
    shapes.set("Disk", new_disk_table(lua)?)?;
    shapes.set("Box", new_box_table(lua)?)?;
    shapes.set("ConstantMedium", new_constant_medium_table(lua)?)?;
    Ok(shapes)
}

fn new_translate_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, object, offset): (Table, Value, AnyUserData)| {
            let object: Hittable = lua.from_value(object)?;
            let offset = from_user_data!(offset, Vec3D);
            let translate = Hittable::Translate(Translate::new(Arc::new(object), offset));
            Ok(lua.to_value(&translate))
        }),
    )
}

fn new_rotate_table(lua: &Lua, kind: RotateKind) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(move |lua, (_, object, angle): (Table, Value, Real)| {
            let object: Hittable = lua.from_value(object)?;
            let rotate_y = Hittable::Rotate(Rotate::new(Arc::new(object), angle, kind.clone()));
            Ok(lua.to_value(&rotate_y))
        }),
    )
}

fn new_transforms_table(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;
    table.set("Translate", new_translate_table(lua)?)?;
    table.set("RotateX", new_rotate_table(lua, RotateKind::X)?)?;
    table.set("RotateY", new_rotate_table(lua, RotateKind::Y)?)?;
    table.set("RotateZ", new_rotate_table(lua, RotateKind::Z)?)?;
    Ok(table)
}

fn new_constant_medium_table(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "from_texture",
        lua.create_function(
            |lua, (_, hittable, density, texture): (Table, Value, Real, Value)| {
                let hittable = lua.from_value(hittable)?;
                let texture = lua.from_value(texture)?;
                let constant_medium = Hittable::ConstantMedium(ConstantMedium::from_texture(
                    hittable, density, texture,
                ));
                Ok(lua.to_value(&constant_medium))
            },
        )?,
    )?;
    table.set(
        "from_albedo",
        lua.create_function(
            |lua, (_, hittable, density, albedo): (Table, Value, Real, AnyUserData)| {
                let hittable = lua.from_value(hittable)?;
                let albedo = from_user_data!(albedo, Color);
                let constant_medium = Hittable::ConstantMedium(ConstantMedium::from_albedo(
                    hittable, density, albedo,
                ));
                Ok(lua.to_value(&constant_medium))
            },
        )?,
    )?;

    Ok(table)
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

fn new_background_table(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "from_color",
        lua.create_function(|lua, (_, color): (Table, AnyUserData)| {
            let color = from_user_data!(color, Color);
            let background = Background::from_color(color);
            Ok(lua.to_value(&background))
        })?,
    )?;
    table.set(
        "from_lerp",
        lua.create_function(|lua, (_, start, end): (Table, AnyUserData, AnyUserData)| {
            let start = from_user_data!(start, Color);
            let end = from_user_data!(end, Color);
            let background = Background::from_lerp(start, end);
            Ok(lua.to_value(&background))
        })?,
    )?;

    Ok(table)
}

fn new_object_list_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, this: Table| {
            let object_list_table = lua.create_table()?;

            object_list_table.set_metatable(Some(this.clone()));
            this.set("__index", this.clone())?;

            object_list_table.set(
                "add",
                lua.create_function(|lua, (this, object): (Table, Value)| {
                    // Let's do a round-trip conversion for now to validate the structure.
                    // This may not be the cleanest solution.
                    let hittable: Hittable = lua.from_value(object)?;

                    let next_index = this.raw_len() + 1;
                    this.set(next_index, lua.to_value(&hittable)?)
                })?,
            )?;
            Ok(object_list_table)
        }),
    )
}

fn new_bvh_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|lua, (_, h_list): (Table, Value)| {
            let hittable_list: HittableList = HittableList::from_vec(lua.from_value(h_list)?);
            let bvh = bvh::BVH::from_list(hittable_list);
            Ok(lua.to_value(&BVH(bvh)))
        }),
    )
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
    engine.set("textures", new_textures_table(lua)?)?;
    engine.set("shapes", new_shapes_table(lua)?)?;
    engine.set("transforms", new_transforms_table(lua)?)?;
    engine.set("Camera", new_camera_table(lua)?)?;
    engine.set("Background", new_background_table(lua)?)?;
    engine.set("ObjectList", new_object_list_table(lua)?)?;
    engine.set("Scene", new_scene_table(lua)?)?;
    engine.set("BVH", new_bvh_table(lua)?)?;

    lua.globals().set("engine", engine)?;

    Ok(())
}
