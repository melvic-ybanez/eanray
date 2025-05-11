use crate::core::math::Real;
use crate::core::math::interval::Interval;
use crate::core::math::vector::{Point, UnitVec3D};
use crate::core::ray::Ray;
use crate::core::shapes::sphere::Sphere;

pub struct HitRecord {
    p: Point,
    normal: UnitVec3D,
    t: Real,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point, normal: UnitVec3D, t: Real, front_face: bool) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn face_normal(ray: &Ray, outward_normal: UnitVec3D) -> (bool, UnitVec3D) {
        let front_face = ray.direction().dot(&outward_normal.0) < 0.0;
        let face_normal = if front_face {
            outward_normal
        } else {
            UnitVec3D(-outward_normal.0)
        };
        (front_face, face_normal)
    }

    pub fn normal(&self) -> &UnitVec3D {
        &self.normal
    }
}

pub enum Hittable {
    Sphere(Sphere),
    List(HittableList),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, ray_t),
            Hittable::List(list) => list.hit(ray, ray_t),
        }
    }
}

pub struct HittableList {
    objects: Vec<Hittable>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn from_vec(objects: Vec<Hittable>) -> HittableList {
        HittableList { objects }
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.objects.iter().fold(None, |maybe_prev_record, object| {
            if let Some(prev_record) = maybe_prev_record {
                object
                    .hit(ray, &Interval::new(ray_t.min(), prev_record.t))
                    .or(Some(prev_record))
            } else {
                object.hit(ray, ray_t)
            }
        })
    }
}
