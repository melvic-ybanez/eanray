use crate::bindings;
use crate::bindings::macros::from_user_data;
use crate::bindings::schemas::{CameraSchema, SceneSchema};
use crate::bindings::{materials, shapes, textures, transforms};
use crate::core::camera::Background;
use crate::core::color::ColorKind;
use crate::core::math::Real;
use crate::core::Hittable::BVH;
use crate::core::{bvh, Color, Hittable, HittableList};
use mlua::{AnyUserData, Function, Lua, LuaSerdeExt, Result, Table, Value};

pub(crate) fn new_table(lua: &Lua, function: Result<Function>) -> Result<Table> {
    let table = lua.create_table()?;
    table.set("new", function?)?;
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

            object_list_table.set_metatable(Some(this.clone()))?;
            this.set("__index", this.clone())?;

            object_list_table.set(
                "add",
                lua.create_function(|lua, (this, object): (Table, AnyUserData)| {
                    let hittable: Hittable = from_user_data!(object, Hittable);

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
            Ok(BVH(bvh))
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
