use crate::core::aabb::AABB;
use crate::core::hittables::{HitRecord, HittableFields};
use crate::core::math::interval::Interval;
use crate::core::math::vector::UnitVec3D;
use crate::core::math::{Point, Real, Vec3D};
use crate::core::shapes::quadrics;
use crate::core::shapes::quadrics::point_within_disk;
use crate::core::{hittables, math, Material, Ray};

#[derive(Clone, Debug)]
pub(crate) struct Cone {
    height: Real,
    base_radius: Real,
    apex_radius: Real,
    end_type: EndType,
    pub(super) fields: HittableFields,
}

impl Cone {
    pub(crate) fn new(
        base_radius: Real,
        apex_radius: Real,
        height: Real,
        material: Material,
        end_type: EndType,
    ) -> Self {
        let mut this = Self {
            base_radius,
            apex_radius,
            height,
            fields: HittableFields::from_mat(material),
            end_type,
        };
        this.compute_bounding_box();
        this
    }

    pub(crate) fn full(
        base_radius: Real,
        height: Real,
        material: Material,
        end_type: EndType,
    ) -> Self {
        Self::new(base_radius, 0.0, height, material, end_type)
    }

    pub(crate) fn frustum(
        base_radius: Real,
        apex_radius: Real,
        height: Real,
        material: Material,
        end_type: EndType,
    ) -> Self {
        Self::new(base_radius, apex_radius, height, material, end_type)
    }

    /// The formula for a cone with base radius `r_base`, apex radius `r_apex` and height
    /// `h` is `x^2 + z^2 = r(y)^2`, where `r(y) = r_apex + (r_base - r_apex) / y` is the slope
    /// of the cone. The said slope is also the ratio of the base radius to the height, or the tangent
    /// of the angle between the y-axis and the side of the cone.
    /// Since the tangent remains the same regardless of `y`, it follows that
    /// `(r(y) - r_apex) / y = (r_base - r_apex) / h`, where `r(y)` is the radius at any `y`.
    ///
    /// Thus, `r(y) = r_apex + (r_base - r_apex) / h * y`. Thus, we get the formula above.
    pub(crate) fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // for brevity, and to mimic the variables above
        let d = ray.direction();
        let o = ray.origin();
        let k = (self.base_radius - self.apex_radius) / self.height;
        let b0 = self.apex_radius + k * o.y;
        let b1 = k * d.y;

        // Expanding the equation into quadratic form, and solving for a, b, and c, yields:
        let a = (d.x * d.x + d.z * d.z) - b1 * b1;
        let b = 2.0 * (o.x * d.x + o.z * d.z - b0 * b1);
        let c = o.x * o.x + o.z * o.z - b0 * b0;

        let t0 = compute_root(a, b, c, ray_t, |root| self.check_y_range(ray, root));
        let (t, hit_type) = nearest_hit(t0, self.nearest_cap_hit(ray, ray_t));
        let compute_mat = || match hit_type {
            HitType::Side => self.fields.material(),
            _ => match &self.end_type {
                EndType::Closed { cap_mat } => &cap_mat,
                _ => self.fields.material(),
            },
        };

        record_hit_details(
            (t, hit_type.clone()),
            ray,
            |p| self.normalized_gradient_vector(p, k),
            |outward_normal| self.compute_uv(outward_normal, hit_type.clone()),
            compute_mat,
        )
    }

    pub fn nearest_cap_hit(&self, ray: &Ray, ray_t: &Interval) -> (Real, HitType) {
        match self.end_type {
            EndType::Closed { .. } => compare_cap_hits(
                self.hit_cap(ray, ray_t, 0.0, self.apex_radius),
                self.hit_cap(ray, ray_t, self.height, self.base_radius),
            ),
            _ => (math::INFINITY, HitType::Side),
        }
    }

    fn hit_cap(&self, ray: &Ray, ray_t: &Interval, height: Real, radius: Real) -> Option<Real> {
        point_within_disk(ray, ray_t, height, radius)
    }

    fn check_y_range(&self, ray: &Ray, t: Real) -> bool {
        let y_hit = ray.origin().y + t * ray.direction().y;
        0.0 <= y_hit && y_hit <= self.height
    }

    /// We'll take the vector of partial derivatives of the cone's formula
    /// `F(x, y, z) = x^2 + z^2 = k^2y^2`. This gives us `(2x, -2k^2y, 2z)`,
    /// which we can simplify to `(x, -k^2y, z)` by removing the scalar since
    /// it will be normalized anyway.
    fn normalized_gradient_vector(&self, p: &Point, k: Real) -> UnitVec3D {
        Vec3D::new(p.x, -k * -k * p.y, p.z).to_unit()
    }

    fn compute_uv(&self, p: &Vec3D, hit_type: HitType) -> (Real, Real) {
        match hit_type {
            HitType::Side => (compute_side_u(p), p.y / self.height),
            HitType::BaseCap => compute_cap_uv(p, self.base_radius),
            HitType::ApexCap => compute_cap_uv(p, self.apex_radius),
        }
    }

    fn compute_bounding_box(&mut self) {
        self.fields.bounding_box = AABB::from_points(
            Point::new(-self.base_radius, 0.0, -self.base_radius),
            Point::new(self.base_radius, self.height, self.base_radius),
        )
    }
}

#[derive(Clone, Debug)]
pub(crate) enum EndType {
    Open,
    Closed { cap_mat: Material },
}

#[derive(Clone)]
pub(crate) enum HitType {
    Side,
    ApexCap,
    BaseCap,
}

pub(super) fn compute_side_u(p: &Vec3D) -> Real {
    let theta = p.z.atan2(p.x); // angle around the y-axis. range: (-pi, pi]
    theta / (2.0 * math::PI) + 0.5 // convert to [0, 1].
}

pub(super) fn compute_cap_uv(p: &Vec3D, radius: Real) -> (Real, Real) {
    // Treat the caps like flat disks, so x and z are elements of [-r, r].
    // Then normalize.
    let u = p.x / (2.0 * radius) + 0.5;
    let v = p.z / (2.0 * radius) + 0.5;
    (u, v)
}

pub(super) fn compute_root<F>(a: Real, b: Real, c: Real, ray_t: &Interval, check_range: F) -> Real
where
    F: Fn(Real) -> bool,
{
    let discriminant = math::discriminant(a, b, c);

    if discriminant < 0.0 {
        math::INFINITY
    } else {
        quadrics::compute_root_from_discriminant(discriminant, a, b, ray_t, |root| {
            check_range(root)
        })
        .unwrap_or(math::INFINITY)
    }
}

pub(super) fn nearest_hit(t0: Real, nearest_cap_hit: (Real, HitType)) -> (Real, HitType) {
    let (t1, hit_type) = nearest_cap_hit;
    if t0 < t1 {
        (t0, HitType::Side)
    } else {
        (t1, hit_type)
    }
}

pub(super) fn compare_cap_hits(hit0: Option<Real>, hit1: Option<Real>) -> (Real, HitType) {
    match (hit0, hit1) {
        (Some(t1), Some(t2)) => {
            if t1 < t2 {
                (t1, HitType::ApexCap)
            } else {
                (t2, HitType::BaseCap)
            }
        }
        (Some(t1), _) => (t1, HitType::ApexCap),
        (_, Some(t2)) => (t2, HitType::BaseCap),
        _ => (math::INFINITY, HitType::Side),
    }
}

pub(super) fn record_hit_details<'a, S, C, M>(
    nearest_cap_hit: (Real, HitType),
    ray: &Ray,
    side_normal: S,
    compute_uv: C,
    compute_mat: M,
) -> Option<HitRecord<'a>>
where
    S: Fn(&Point) -> UnitVec3D,
    C: Fn(&UnitVec3D) -> (Real, Real),
    M: Fn() -> &'a Material,
{
    let (t, hit_type) = nearest_cap_hit;
    if t == math::INFINITY {
        None
    } else {
        let p = ray.at(t);
        let outward_normal = match hit_type {
            HitType::Side => side_normal(&p),
            HitType::BaseCap => Vec3D::new(0.0, 1.0, 0.0).to_unit(),
            HitType::ApexCap => Vec3D::new(0.0, -1.0, 0.0).to_unit(),
        };
        let (u, v) = compute_uv(&outward_normal);
        let (front_face, face_normal) = HitRecord::face_normal(&ray, outward_normal);

        Some(HitRecord::new(
            hittables::HitPoint(p),
            hittables::Normal(face_normal),
            hittables::Mat(compute_mat()),
            hittables::T(t),
            hittables::FrontFace(front_face),
            hittables::U(u),
            hittables::V(v),
        ))
    }
}
