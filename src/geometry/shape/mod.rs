use crate::*;
mod sphere;
mod transform;
mod triangle;
use downcast_rs::DowncastSync;
pub use sphere::*;
pub use transform::*;
pub use triangle::*;

pub trait Shape: DowncastSync {
    fn area(&self) -> Float;
    fn bound(&self) -> Bounds3f;
    fn sample(&self, sampler: &mut dyn Sampler) -> (ShapePoint, Float);
    fn pdf(&self, _: &ShapePoint) -> Float {
        1. / self.area()
    }
    fn sample_by_point(&self, point: &Point3f, sampler: &mut dyn Sampler) -> (ShapePoint, Float) {
        self.default_sample_by_point(point, sampler)
    }
    fn default_sample_by_point(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
    ) -> (ShapePoint, Float) {
        let (shape_point, pdf) = self.sample(sampler);
        let wi = shape_point.p - point;
        if wi.magnitude_squared() == 0. {
            (shape_point, 0.)
        } else {
            let pdf = pdf * wi.magnitude_squared() / -wi.normalize().dot(&shape_point.n);
            if pdf.is_nan() || pdf.is_infinite() {
                (shape_point, 0.)
            } else {
                (shape_point, pdf)
            }
        }
    }
    fn default_by_point_pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float {
        let d = point - shape_point.p;
        let distance_2 = d.magnitude_squared();
        let distance = distance_2.sqrt();
        let pdf = distance_2 / ((d / distance).dot(&shape_point.n).abs() * self.area());
        if pdf.is_nan() || pdf.is_infinite() {
            0.
        }
        else {
            pdf
        }
    }
    fn by_point_pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float {
        self.default_by_point_pdf(point, shape_point)
    }
    fn by_point_w_pdf(&self, point: &Point3f, w: &Vector3f) -> Float {
        let ray = Ray::new_od(*point, *w);
        if let Some(intersect) = self.intersect(&ray) {
            self.by_point_pdf(point, intersect.get_shape_point())
        } else {
            0.
        }
    }
    fn intersect(&self, ray: &Ray) -> Option<ShapeIntersect>;
    fn intersect_through_bound(&self, ray: &RayIntersectCache) -> Option<ShapeIntersect> {
        if self.bound().intersect_predicate_cached(ray) {
            self.intersect(ray.origin_ray())
        } else {
            None
        }
    }
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
    fn intersect_predicate_through_bound(&self, ray: &RayIntersectCache) -> bool {
        if self.bound().intersect_predicate_cached(ray) {
            self.intersect_predicate(ray.origin_ray())
        } else {
            false
        }
    }
    fn box_clone(&self) -> Box<dyn Shape>;
}

impl_downcast!(sync Shape);

pub fn shape_apply(shape: Box<dyn Shape>, transform: &Transform) -> Box<dyn Shape> {
    match shape.downcast::<TransformShape>() {
        Ok(transfrom_shape) => Box::new(transfrom_shape.apply(transform)),
        Err(shape) => Box::new(TransformShape::from(shape).apply(transform)),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ShapePoint {
    pub p: Point3f,
    pub n: Normal3f,
    pub uv: Point2f,
    p_error: Vector3f,
}

impl ShapePoint {
    pub fn new_p_normal(p: Point3f, n: Normal3f) -> Self {
        Self::new(p, n, Point2f::new(0., 0.), Vector3f::new(0., 0., 0.))
    }
    pub fn new_p_normal_error(p: Point3f, n: Normal3f, p_error: Vector3f) -> Self {
        Self::new(p, n, Point2f::new(0., 0.), p_error)
    }
    pub fn new(p: Point3f, n: Normal3f, uv: Point2f, p_error: Vector3f) -> Self {
        Self { p, n, uv, p_error }
    }
    pub fn point_offset_by_error(&self, w: &Vector3f) -> Point3f {
        let d: Float = self.n.as_ref().abs().dot(&self.p_error);
        let mut offset: Vector3f = self.n.as_ref() * d;
        if w.dot(&self.n) < 0. {
            offset = -offset;
        }
        self.p + offset
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ShapeIntersect {
    t: Float,
    p: ShapePoint,
}

impl Transformable for ShapePoint {
    fn apply(self, transform: &super::Transform) -> Self {
        let p = self.p.apply(transform);
        let n = self.n.apply(transform);
        let m = &transform.m;
        let x_abs_sum = (m.row(0)[0] * p.x).abs()
            + (m.row(0)[1] * p.y).abs()
            + (m.row(0)[2] * p.z).abs()
            + m.row(0)[3];
        let y_abs_sum = (m.row(1)[0] * p.x).abs()
            + (m.row(1)[1] * p.y).abs()
            + (m.row(1)[2] * p.z).abs()
            + m.row(1)[3];
        let z_abs_sum = (m.row(2)[0] * p.x).abs()
            + (m.row(2)[1] * p.y).abs()
            + (m.row(2)[2] * p.z).abs()
            + m.row(2)[3];
        let p_error = gamma(3) * Vector3f::new(x_abs_sum, y_abs_sum, z_abs_sum);
        Self {
            p,
            n,
            uv: self.uv,
            p_error,
        }
    }
}
impl ShapeIntersect {
    pub fn new(p: Point3f, n: Normal3f, t: Float, uv: Point2f, p_error: Vector3f) -> Self {
        Self {
            t,
            p: ShapePoint::new(p, n, uv, p_error),
        }
    }
    pub fn from_shape_point(p: ShapePoint, t: Float) -> Self {
        Self { t, p }
    }
    pub fn get_point(&self) -> &Point3f {
        &self.p.p
    }
    pub fn get_normal(&self) -> &Normal3f {
        &self.p.n
    }
    pub fn get_uv(&self) -> &Point2f {
        &self.p.uv
    }
    pub fn get_shape_point(&self) -> &ShapePoint {
        &self.p
    }

    pub fn get_t(&self) -> Float {
        self.t
    }
}

impl Transformable for ShapeIntersect {
    fn apply(self, transform: &Transform) -> Self {
        Self {
            t: self.t,
            p: self.p.apply(transform),
        }
    }
}

pub fn parse_shape(property_set: &PropertySet) -> Vec<Box<dyn Shape>> {
    match property_set.get_name().unwrap() {
        "sphere" => {
            let radius = property_set.get_value("radius").unwrap_or(1.);
            vec![Box::new(Sphere::new(radius))]
        }
        "trianglemesh" => {
            let indices = property_set.get_value("indices").unwrap();
            let vertices = property_set.get_value("P").unwrap();
            let triangle_mesh = TriangleMesh::new(indices, vertices).into();
            create_triangles(triangle_mesh)
        }
        _ => panic!(),
    }
}
