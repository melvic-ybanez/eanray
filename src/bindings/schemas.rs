use std::io;
use mlua::{LuaSerdeExt, MetaMethod, UserData, UserDataMethods, Value};
use serde::{Deserialize, Serialize};
use crate::core::{Camera, Color, Hittable, HittableList};
use crate::core::camera::Image;
use crate::core::math::{Point, Real, Vec3D, VecLike};
use crate::core::math::vector::CanAdd;
use crate::settings;
use crate::settings::Config;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SceneSchema<'a> {
    camera: CameraSchema,
    objects: Vec<Hittable<'a>>,
}

impl<'a> SceneSchema<'a> {
    pub fn new(camera: CameraSchema, objects: Vec<Hittable<'a>>) -> Self {
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
pub struct CameraSchema {
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
    pub fn new(aspect_ratio: Real, image_width: u32) -> Self {
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