use crate::core::aabb::AABB;
use crate::core::bvh::BVH;
use crate::core::materials::Material;
use crate::core::math;
use crate::core::math::interval::Interval;
use crate::core::math::ray::Ray;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::shapes::planars::Planar;
use crate::core::shapes::plane::Plane;
use crate::core::shapes::quadrics::Quadric;
use crate::core::shapes::volume::ConstantMedium;
use crate::core::transform::Transform;
use crate::diagnostics::metrics;
use std::sync::Arc;

pub(crate) type ObjectRef = Arc<Hittable>;

pub(crate) struct HitRecord<'a> {
    pub(crate) hit_point: Point,
    pub(crate) normal: UnitVec3D,
    mat: &'a Material,
    pub(crate) t: Real,
    front_face: bool,
    u: Real,
    v: Real,
}

impl<'a> HitRecord<'a> {
    pub(crate) fn new(
        hit_point: HitPoint,
        normal: Normal,
        mat: Mat<'a>,
        t: T,
        front_face: FrontFace,
        u: U,
        v: V,
    ) -> HitRecord {
        HitRecord {
            hit_point: hit_point.0,
            normal: normal.0,
            mat: mat.0,
            t: t.0,
            front_face: front_face.0,
            u: u.0,
            v: v.0,
        }
    }

    pub(crate) fn face_normal(ray: &Ray, outward_normal: UnitVec3D) -> (bool, UnitVec3D) {
        let front_face = ray.direction().dot(&outward_normal.0) < 0.0;
        let face_normal = if front_face {
            outward_normal
        } else {
            UnitVec3D(-outward_normal.0)
        };
        (front_face, face_normal)
    }

    pub(crate) fn p(&self) -> &Point {
        &self.hit_point
    }

    pub(crate) fn normal(&self) -> &UnitVec3D {
        &self.normal
    }

    pub(crate) fn material(&self) -> &Material {
        &self.mat
    }

    pub(crate) fn front_face(&self) -> bool {
        self.front_face
    }

    pub(crate) fn t(&self) -> Real {
        self.t
    }

    pub(crate) fn u(&self) -> Real {
        self.u
    }

    pub(crate) fn v(&self) -> Real {
        self.v
    }
}

pub(crate) struct HitPoint(pub(crate) Point);
pub(crate) struct Normal(pub(crate) UnitVec3D);
pub(crate) struct Mat<'a>(pub(crate) &'a Material);
pub(crate) struct T(pub(crate) Real);
pub(crate) struct FrontFace(pub(crate) bool);
pub(crate) struct U(pub(crate) Real);
pub(crate) struct V(pub(crate) Real);

#[derive(Clone, Debug)]
pub(crate) enum Hittable {
    Quadric(Quadric),
    List(HittableList),
    BVH(BVH),
    Planar(Planar),
    ConstantMedium(ConstantMedium),
    Plane(Plane),
    Transform(Transform),
}

impl Hittable {
    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if self.has_geometry() {
            metrics::increment_object_hit_attempt_count();
        }

        match self {
            Self::Transform(transform) => {
                let transformed_ray = ray.transform(transform.inverse());

                if let Some(mut hit_record) = transform
                    .object
                    .hit_with_transformed_ray(&transformed_ray, ray_t)
                {
                    hit_record.hit_point = hit_record.hit_point.transform(transform.forward());
                    hit_record.normal = hit_record.normal.transform(transform.normal()).to_unit();
                    Some(hit_record)
                } else {
                    None
                }
            }
            _ => self.hit_with_transformed_ray(ray, ray_t),
        }
    }

    fn hit_with_transformed_ray(
        &self,
        transformed_ray: &Ray,
        ray_t: &Interval,
    ) -> Option<HitRecord> {
        match self {
            Self::Quadric(quadric) => quadric.hit(transformed_ray, ray_t),
            Self::List(list) => list.hit(transformed_ray, ray_t),
            Self::BVH(bvh) => bvh.hit(transformed_ray, ray_t),
            Self::Planar(planar) => planar.hit(transformed_ray, ray_t),
            Self::ConstantMedium(constant_medium) => constant_medium.hit(transformed_ray, ray_t),
            Self::Plane(plane) => plane.hit(transformed_ray, ray_t),
            _ => None,
        }
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        match self {
            Self::Quadric(quadric) => quadric.fields().bounding_box(),
            Self::List(list) => list.bounding_box(),
            Self::BVH(bvh) => bvh.bounding_box(),
            Self::Planar(planar) => &planar.fields.bounding_box,
            Self::ConstantMedium(constant_medium) => constant_medium.bounding_box(),
            Self::Plane(plane) => &plane.fields.bounding_box,
            Self::Transform(transform) => transform.bounding_box(),
        }
    }

    pub(crate) fn is_finite(&self) -> bool {
        let bbox = self.bounding_box();
        bbox.x().min < math::INFINITY
            && bbox.x().max < math::INFINITY
            && bbox.y().min < math::INFINITY
            && bbox.y().max < math::INFINITY
            && bbox.z().min < math::INFINITY
            && bbox.z().max < math::INFINITY
    }

    pub(crate) fn has_geometry(&self) -> bool {
        match self {
            Self::BVH(_) | Self::List(_) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HittableFields {
    pub(crate) bounding_box: AABB,
    mat: Material,
}

impl HittableFields {
    pub(crate) fn new(mat: Material, bounding_box: AABB) -> Self {
        Self {
            mat,
            bounding_box,
        }
    }

    pub(crate) fn from_mat(mat: Material) -> Self {
        Self::new(mat, AABB::empty())
    }

    pub(crate) fn material(&self) -> &Material {
        &self.mat
    }

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

#[derive(Clone, Debug)]
pub(crate) struct HittableList {
    objects: Vec<Hittable>,
    bbox: AABB,
}

impl HittableList {
    pub(crate) fn empty() -> Self {
        Self::from_vec(vec![])
    }

    pub(crate) fn from_vec(objects: Vec<Hittable>) -> HittableList {
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

    pub(crate) fn add(&mut self, object: Hittable) {
        self.bbox = AABB::from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }

    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
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

    pub(crate) fn make_box(a: Point, b: Point, mat: Material) -> Self {
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

    pub(crate) fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub(crate) fn objects(&self) -> &[Hittable] {
        &self.objects
    }

    pub(crate) fn objects_mut(&mut self) -> &mut Vec<Hittable> {
        &mut self.objects
    }
}
