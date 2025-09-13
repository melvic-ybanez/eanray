use crate::bindings::macros::from_user_data;
use crate::core::camera::{Background, Image};
use crate::core::math::{Point, Real, Vec3D, VecLike};
use crate::core::{Camera, Color, Hittable, HittableList};
use crate::settings;
use crate::settings::Config;
use mlua::{AnyUserData, UserData, UserDataFields};
use std::io;

#[derive(Clone, Debug)]
pub(crate) struct SceneSchema {
    camera: CameraSchema,
    objects: HittableList,
}

impl SceneSchema {
    pub(crate) fn new(camera: CameraSchema, objects: HittableList) -> Self {
        Self { camera, objects }
    }

    pub(crate) fn render(&self, config: &'static Config) -> io::Result<()> {
        let camera = self.camera.build(config);
        camera.render(&Hittable::List(self.objects.clone()), config)
    }
}

impl UserData for SceneSchema {}

#[derive(Clone, Debug)]
pub(crate) struct CameraSchema {
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
    background: Option<Background>,
    vup: Option<Vec3D>,
}

impl CameraSchema {
    pub(crate) fn new(aspect_ratio: Real, image_width: u32) -> Self {
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
            background: None,
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
            .background({
                let default = defaults.background();
                self.background
                    .clone()
                    .unwrap_or(Background::from_color(Color::new(
                        default[0], default[1], default[2],
                    )))
            })
            .build()
    }
}

impl UserData for CameraSchema {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_set("aspect_ratio", |_, this, aspect_ratio| {
            Ok(this.aspect_ratio = aspect_ratio)
        });
        fields.add_field_method_set("image_width", |_, this, image_width| {
            Ok(this.image_width = image_width)
        });
        fields.add_field_method_set("samples_per_pixel", |_, this, samples_per_pixel| {
            Ok(this.samples_per_pixel = Some(samples_per_pixel))
        });
        fields.add_field_method_set("antialiasing", |_, this, antialiasing| {
            Ok(this.antialiasing = Some(antialiasing))
        });
        fields.add_field_method_set("max_depth", |_, this, max_depth| {
            Ok(this.max_depth = Some(max_depth))
        });
        fields.add_field_method_set("field_of_view", |_, this, field_of_view| {
            Ok(this.field_of_view = Some(field_of_view))
        });
        fields.add_field_method_set("look_from", |_, this, look_from: AnyUserData| {
            let look_from = from_user_data!(look_from, Point);
            Ok(this.look_from = Some(look_from))
        });
        fields.add_field_method_set("look_at", |_, this, look_at: AnyUserData| {
            let look_at = from_user_data!(look_at, Point);
            Ok(this.look_at = Some(look_at))
        });
        fields.add_field_method_set("defocus_angle", |_, this, defocus_angle| {
            Ok(this.defocus_angle = Some(defocus_angle))
        });
        fields.add_field_method_set("focus_distance", |_, this, focus_distance| {
            Ok(this.focus_distance = Some(focus_distance))
        });
        fields.add_field_method_set("background", |_, this, background: AnyUserData| {
            let background = from_user_data!(background, Background);
            Ok(this.background = Some(background))
        });
        fields.add_field_method_set("vup", |_, this, vup: AnyUserData| {
            let vup = from_user_data!(vup, Vec3D);
            Ok(this.vup = Some(vup))
        });
    }
}
