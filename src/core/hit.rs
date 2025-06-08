use crate::core::materials::Material;
use crate::core::math::interval::Interval;
use crate::core::math::vector::{Point, UnitVec3D};
use crate::core::math::Real;
use crate::core::ray::Ray;
use crate::core::shapes::sphere::Sphere;
use serde::{Deserialize, Serialize};

pub struct HitRecord<'a> {
    p: Point,
    normal: UnitVec3D,
    mat: &'a Material,
    t: Real,
    front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: P, normal: Normal, mat: Mat<'a>, t: T, front_face: FrontFace) -> HitRecord {
        HitRecord {
            p: p.0,
            normal: normal.0,
            mat: mat.0,
            t: t.0,
            front_face: front_face.0,
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

    pub fn p(&self) -> &Point {
        &self.p
    }

    pub fn normal(&self) -> &UnitVec3D {
        &self.normal
    }

    pub fn material(&self) -> &Material {
        &self.mat
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub struct P(pub Point);
pub struct Normal(pub UnitVec3D);
pub struct Mat<'a>(pub &'a Material);
pub struct T(pub Real);
pub struct FrontFace(pub bool);

// TODO: See if we can just use HittableList in all cases and drop the Sphere variant
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
