use crate::core::aabb::AABB;
use crate::core::bvh::BVH;
use crate::core::materials::Material;
use crate::core::math;
use crate::core::math::interval::Interval;
use crate::core::math::vector::{Point, UnitVec3D};
use crate::core::math::{Real, Vec3D};
use crate::core::ray::Ray;
use crate::core::shapes::planar::Planar;
use crate::core::shapes::plane::Plane;
use crate::core::shapes::sphere::Sphere;
use crate::core::shapes::volume::ConstantMedium;
use crate::core::transforms::{Rotate, Translate};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type ObjectRef = Arc<Hittable>;

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: UnitVec3D,
    mat: &'a Material,
    pub t: Real,
    front_face: bool,
    u: Real,
    v: Real,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: P,
        normal: Normal,
        mat: Mat<'a>,
        t: T,
        front_face: FrontFace,
        u: U,
        v: V,
    ) -> HitRecord {
        HitRecord {
            p: p.0,
            normal: normal.0,
            mat: mat.0,
            t: t.0,
            front_face: front_face.0,
            u: u.0,
            v: v.0,
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

    pub fn t(&self) -> Real {
        self.t
    }

    pub fn u(&self) -> Real {
        self.u
    }

    pub fn v(&self) -> Real {
        self.v
    }
}

pub struct P(pub Point);
pub struct Normal(pub UnitVec3D);
pub struct Mat<'a>(pub &'a Material);
pub struct T(pub Real);
pub struct FrontFace(pub bool);
pub struct U(pub Real);
pub struct V(pub Real);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Hittable {
    Sphere(Sphere),
    List(HittableList),
    BVH(BVH),
    Planar(Planar),
    Translate(Translate),
    Rotate(Rotate),
    ConstantMedium(ConstantMedium),
    Plane(Plane),
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, ray_t),
            Hittable::List(list) => list.hit(ray, ray_t),
            Hittable::BVH(bvh) => bvh.hit(ray, ray_t),
            Hittable::Planar(quad) => quad.hit(ray, ray_t),
            Hittable::Translate(translate) => translate.hit(ray, ray_t),
            Hittable::Rotate(rotate_y) => rotate_y.hit(ray, ray_t),
            Hittable::ConstantMedium(constant_medium) => constant_medium.hit(ray, ray_t),
            Hittable::Plane(plane) => plane.hit(ray, ray_t),
        }
    }

    pub fn bounding_box(&self) -> &AABB {
        match self {
            Hittable::Sphere(sphere) => sphere.bounding_box(),
            Hittable::List(list) => list.bounding_box(),
            Hittable::BVH(bvh) => bvh.bounding_box(),
            Hittable::Planar(quad) => quad.bounding_box(),
            Hittable::Translate(translate) => translate.bounding_box(),
            Hittable::Rotate(rotate_y) => rotate_y.bounding_box(),
            Hittable::ConstantMedium(constant_medium) => constant_medium.bounding_box(),
            Hittable::Plane(plane) => plane.bounding_box(),
        }
    }

    pub fn is_finite(&self) -> bool {
        let bbox = self.bounding_box();
        bbox.x().min < math::INFINITY
            && bbox.x().max < math::INFINITY
            && bbox.y().min < math::INFINITY
            && bbox.y().max < math::INFINITY
            && bbox.z().min < math::INFINITY
            && bbox.z().max < math::INFINITY
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HittableList {
    objects: Vec<Hittable>,
    bbox: AABB,
}

impl HittableList {
    pub fn empty() -> Self {
        Self::from_vec(vec![])
    }

    pub fn from_vec(objects: Vec<Hittable>) -> HittableList {
        let mut this = Self {
            objects: vec![],
            bbox: AABB::empty(),
        };

        // call `add` for each item to update the bbox
        for object in objects {
            this.add(object);
        }
        this
    }

    pub fn add(&mut self, object: Hittable) {
        self.bbox = AABB::from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.objects.iter().fold(None, |maybe_prev_record, object| {
            if let Some(prev_record) = maybe_prev_record {
                object
                    .hit(ray, &mut Interval::new(ray_t.min, prev_record.t))
                    .or(Some(prev_record))
            } else {
                object.hit(ray, ray_t)
            }
        })
    }

    pub fn make_box(a: Point, b: Point, mat: Material) -> Self {
        let min = Point::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vec3D::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3D::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3D::new(0.0, 0.0, max.z - min.z);

        let sides = vec![
            Planar::quad(
                Point::new(min.x, min.y, max.z),
                dx.clone(),
                dy.clone(),
                mat.clone(),
            ), // front
            Planar::quad(
                Point::new(max.x, min.y, min.z),
                -dx.clone(),
                dy.clone(),
                mat.clone(),
            ), // back
            Planar::quad(
                Point::new(min.x, min.y, min.z),
                dz.clone(),
                dy.clone(),
                mat.clone(),
            ), // left
            Planar::quad(
                Point::new(max.x, min.y, max.z),
                -dz.clone(),
                dy.clone(),
                mat.clone(),
            ), // right
            Planar::quad(
                Point::new(min.x, max.y, max.z),
                dx.clone(),
                -dz.clone(),
                mat.clone(),
            ), // top
            Planar::quad(
                Point::new(min.x, min.y, min.z),
                dx.clone(),
                dz.clone(),
                mat.clone(),
            ), // bottom
        ];

        Self::from_vec(
            sides
                .iter()
                .map(|side| Hittable::Planar(side.clone()))
                .collect(),
        )
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub fn objects(&self) -> &[Hittable] {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<Hittable> {
        &mut self.objects
    }
}
