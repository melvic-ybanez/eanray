use crate::bindings;
use crate::bindings::macros::from_user_data;
use crate::bindings::schemas::{CameraSchema, SceneSchema};
use crate::bindings::{materials, shapes, textures, transforms};
use crate::core::camera::Background;
use crate::core::color::ColorKind;
use crate::core::math::Real;
use crate::core::Hittable::BVH;
use crate::core::{bvh, Color, HittableList};
use mlua::{AnyUserData, Function, Lua, Result, Table};

pub(crate) fn new_table(lua: &Lua, function: Result<Function>) -> Result<Table> {
    let table = lua.create_table()?;
    table.set("new", function?)?;
    Ok(table)
}

fn new_camera_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|_, (_, image_width, aspect_ratio): (Table, u32, Real)| {
            Ok(CameraSchema::new(aspect_ratio, image_width))
        }),
    )
}

fn new_background_table(lua: &Lua) -> Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "from_color",
        lua.create_function(|_, (_, color): (Table, AnyUserData)| {
            let color = from_user_data!(color, Color);
            Ok(Background::from_color(color))
        })?,
    )?;
    table.set(
        "from_lerp",
        lua.create_function(|_, (_, start, end): (Table, AnyUserData, AnyUserData)| {
            let start = from_user_data!(start, Color);
            let end = from_user_data!(end, Color);
            let background = Background::from_lerp(start, end);
            Ok(background)
        })?,
    )?;

    Ok(table)
}

fn new_object_list_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|_, _: Table| Ok(HittableList::empty())),
    )
}

fn new_bvh_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|_, (_, h_list): (Table, AnyUserData)| {
            let hittable_list = from_user_data!(h_list, HittableList);
            let bvh = bvh::BVH::from_list(hittable_list);
            Ok(BVH(bvh))
        }),
    )
}

fn new_scene_table(lua: &Lua) -> Result<Table> {
    new_table(
        lua,
        lua.create_function(|_, (_, camera, objects): (Table, AnyUserData, AnyUserData)| {
            let camera: CameraSchema = from_user_data!(camera, CameraSchema);
            let objects = from_user_data!(objects, HittableList);
            let scene: SceneSchema = SceneSchema::new(camera, objects);
            Ok(scene)
        }),
    )
}

pub(crate) fn new_color_table(lua: &Lua) -> Result<Table> {
    let table = bindings::math::new_vec_like_table::<ColorKind>(lua)?;

    table.set("WHITE", Color::white())?;
    table.set("BLACK", Color::black())?;
    table.set("RED", Color::red())?;
    table.set("BLUE", Color::blue())?;
    table.set("GREEN", Color::green())?;

    Ok(table)
}

pub(crate) fn set_engine(lua: &Lua) -> Result<()> {
    let engine = lua.create_table()?;

    engine.set("math", bindings::math::new_table(lua)?)?;
    engine.set("Color", new_color_table(lua)?)?;
    engine.set("materials", materials::new_table(lua)?)?;
    engine.set("textures", textures::new_table(lua)?)?;
    engine.set("shapes", shapes::new_table(lua)?)?;
    engine.set("transforms", transforms::new_table(lua)?)?;
    engine.set("Camera", new_camera_table(lua)?)?;
    engine.set("Background", new_background_table(lua)?)?;
    engine.set("ObjectList", new_object_list_table(lua)?)?;
    engine.set("Scene", new_scene_table(lua)?)?;
    engine.set("BVH", new_bvh_table(lua)?)?;

    lua.globals().set("engine", engine)?;

    Ok(())
}
