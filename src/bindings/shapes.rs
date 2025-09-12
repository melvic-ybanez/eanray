use crate::bindings::lua;
use crate::bindings::macros::from_user_data;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::shapes::planars::{Planar, Quad, Triangle};
use crate::core::shapes::plane::Plane;
use crate::core::shapes::quadrics::cone::{Cone, EndType};
use crate::core::shapes::quadrics::cylinder::Cylinder;
use crate::core::shapes::quadrics::Quadric;
use crate::core::shapes::volume::ConstantMedium;
use crate::core::shapes::{planars, Sphere};
use crate::core::{Color, Hittable, HittableList, Material};
use mlua::{AnyUserData, Lua, LuaSerdeExt, Table, Value};
use std::sync::Arc;

pub(crate) fn new_table(lua: &Lua) -> mlua::Result<Table> {
    let shapes = lua.create_table()?;

    shapes.set("Sphere", new_sphere_table(lua)?)?;
    shapes.set("Quad", new_planar_table(lua, planars::Kind::Quad(Quad))?)?;
    shapes.set(
        "Triangle",
        new_planar_table(lua, planars::Kind::Triangle(Triangle))?,
    )?;
    shapes.set("Disk", new_disk_table(lua)?)?;
    shapes.set("Box", new_box_table(lua)?)?;
    shapes.set("ConstantMedium", new_constant_medium_table(lua)?)?;
    shapes.set("Plane", new_plane_table(lua)?)?;
    shapes.set("Cylinder", new_cylinder_table(lua)?)?;
    shapes.set("Cone", new_cone_table(lua)?)?;

    Ok(shapes)
}

fn new_sphere_table(lua: &Lua) -> mlua::Result<Table> {
    let new_function = lua.create_function(
        |lua, (_, center, radius, material): (Table, AnyUserData, Real, Value)| {
            let center = from_user_data!(center, Point);
            let material: Material = lua.from_value(material)?;
            let sphere = Hittable::Quadric(Quadric::Sphere(Sphere::stationary(
                center, radius, material,
            )));
            Ok(sphere)
        },
    );
    let table = lua::new_table(lua, new_function.clone())?;
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
                let sphere = Hittable::Quadric(Quadric::Sphere(Sphere::moving(
                    center1, center2, radius, material,
                )));
                Ok(sphere)
            },
        )?,
    )?;
    Ok(table)
}

fn new_planar_table(lua: &Lua, kind: planars::Kind) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(
            move |lua, (_, q, u, v, mat): (Table, AnyUserData, AnyUserData, AnyUserData, Value)| {
                let q = from_user_data!(q, Point);
                let u = from_user_data!(u, Vec3D);
                let v = from_user_data!(v, Vec3D);
                let mat: Material = lua.from_value(mat)?;
                let planar = Hittable::Planar(Planar::new(q, u, v, mat, kind.clone()));
                Ok(planar)
            },
        ),
    )
}

fn new_disk_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
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

fn new_box_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(
            |lua, (_, a, b, mat): (Table, AnyUserData, AnyUserData, Value)| {
                let a = from_user_data!(a, Point);
                let b = from_user_data!(b, Point);
                let mat = lua.from_value(mat)?;
                let hl_box = Hittable::List(HittableList::make_box(a, b, mat));
                Ok(hl_box)
            },
        ),
    )
}

fn new_plane_table(lua: &Lua) -> mlua::Result<Table> {
    lua::new_table(
        lua,
        lua.create_function(
            move |lua, (_, p0, n, mat): (Table, AnyUserData, AnyUserData, Value)| {
                let p0 = from_user_data!(p0, Point);
                let n = from_user_data!(n, Vec3D);
                let mat: Material = lua.from_value(mat)?;
                let plane = Hittable::Plane(Plane::new(p0, n.to_unit(), mat));
                Ok(lua.to_value(&plane))
            },
        ),
    )
}

fn new_constant_medium_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "from_texture",
        lua.create_function(
            |lua, (_, hittable, density, texture): (Table, AnyUserData, Real, Value)| {
                let hittable = from_user_data!(hittable, Hittable);
                let texture = lua.from_value(texture)?;
                let constant_medium = Hittable::ConstantMedium(ConstantMedium::from_texture(
                    Arc::new(hittable),
                    density,
                    texture,
                ));
                Ok(constant_medium)
            },
        )?,
    )?;
    table.set(
        "from_albedo",
        lua.create_function(
            |lua, (_, hittable, density, albedo): (Table, AnyUserData, Real, AnyUserData)| {
                let hittable = from_user_data!(hittable, Hittable);
                let albedo = from_user_data!(albedo, Color);
                let constant_medium = Hittable::ConstantMedium(ConstantMedium::from_albedo(
                    Arc::new(hittable),
                    density,
                    albedo,
                ));
                Ok(constant_medium)
            },
        )?,
    )?;

    Ok(table)
}

fn new_cylinder_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "infinite",
        lua.create_function(|lua, (_, radius, material): (Table, Real, Value)| {
            let mat = lua.from_value(material)?;
            let cylinder = Hittable::Quadric(Quadric::Cylinder(Cylinder::infinite(radius, mat)));
            Ok(cylinder)
        })?,
    )?;

    table.set(
        "open",
        lua.create_function(
            |lua, (_, radius, height, material): (Table, Real, Real, Value)| {
                let mat = lua.from_value(material)?;
                let cylinder =
                    Hittable::Quadric(Quadric::Cylinder(Cylinder::open(radius, height, mat)));
                Ok(cylinder)
            },
        )?,
    )?;

    table.set(
        "closed",
        lua.create_function(
            |lua, (_, radius, height, side_mat, cap_mat): (Table, Real, Real, Value, Value)| {
                let side_mat = lua.from_value(side_mat)?;
                let cap_mat = lua.from_value(cap_mat)?;
                let cylinder = Hittable::Quadric(Quadric::Cylinder(Cylinder::closed(
                    radius, height, side_mat, cap_mat,
                )));
                Ok(cylinder)
            },
        )?,
    )?;

    Ok(table)
}

fn new_cone_table(lua: &Lua) -> mlua::Result<Table> {
    let table = lua.create_table()?;

    table.set(
        "full_open",
        lua.create_function(
            |lua, (_, base_radius, height, material): (Table, Real, Real, Value)| {
                let mat = lua.from_value(material)?;
                let cone = Hittable::Quadric(Quadric::Cone(Cone::full(
                    base_radius,
                    height,
                    mat,
                    EndType::Open,
                )));
                Ok(cone)
            },
        )?,
    )?;

    table.set(
        "full_closed",
        lua.create_function(
            |lua, (_, base_radius, height, side_mat, cap_mat): (Table, Real, Real, Value, Value)| {
                let side_mat = lua.from_value(side_mat)?;
                let cap_mat = lua.from_value(cap_mat)?;
                let cone = Hittable::Quadric(Quadric::Cone(Cone::full(
                    base_radius,
                    height,
                    side_mat,
                    EndType::Closed { cap_mat },
                )));
                Ok(cone)
            },
        )?,
    )?;

    table.set(
        "frustum_open",
        lua.create_function(
            |lua, (_, base_radius, apex_radius, height, material): (Table, Real, Real, Real, Value)| {
                let mat = lua.from_value(material)?;
                Ok(Hittable::Quadric(Quadric::Cone(Cone::frustum(base_radius, apex_radius, height, mat, EndType::Open))))
            },
        )?,
    )?;

    table.set(
        "frustum_closed",
        lua.create_function(
            |lua,
             (_, base_radius, apex_radius, height, side_mat, cap_mat): (
                Table,
                Real,
                Real,
                Real,
                Value,
                Value,
            )| {
                let side_mat = lua.from_value(side_mat)?;
                let cap_mat = lua.from_value(cap_mat)?;
                let cone = Hittable::Quadric(Quadric::Cone(Cone::frustum(
                    base_radius,
                    apex_radius,
                    height,
                    side_mat,
                    EndType::Closed { cap_mat },
                )));
                Ok(cone)
            },
        )?,
    )?;

    Ok(table)
}
