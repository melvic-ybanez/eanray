use crate::bindings::schemas::{CameraSchema, SceneSchema};
use crate::core::camera::Background;
use crate::core::color::ColorKind;
use crate::core::math::Real;
use crate::core::Hittable::BVH;
use crate::core::{bvh, Color, Hittable, HittableList};
use mlua::{AnyUserData, Function, Lua, LuaSerdeExt, Result, Table, Value};


macro_rules! from_user_data {
    ($name: ident, $t: ty) => {
        $name.borrow::<$t>()?.clone()
    };
}

pub(in crate::bindings) use from_user_data;
use crate::bindings;
use crate::bindings::{materials, shapes, textures, transforms};

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

    engine.set("math", bindings::math::new_table(lua)?)?;
    engine.set("Color", bindings::math::new_vec_like_table::<ColorKind>(lua)?)?;
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
