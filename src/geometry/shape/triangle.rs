use crate::*;
use std::sync::Arc;
#[derive(Debug)]
pub struct TriangleMesh {
    n_indices: usize,
    n_vertices: usize,
    indices: Vec<usize>,
    vertices: Vec<Point3f>,
    normals: Option<Vec<Normal3f>>,
    uvs: Option<Vec<Point2f>>,
}
impl TriangleMesh {
    pub fn new(
        indices: Vec<usize>,
        vertices: Vec<Point3f>,
        normals: Option<Vec<Normal3f>>,
        uvs: Option<Vec<Point2f>>,
    ) -> Self {
        Self {
            n_indices: indices.len(),
            n_vertices: vertices.len(),
            indices,
            vertices,
            normals,
            uvs,
        }
    }
}

pub fn create_triangles(mesh: Arc<TriangleMesh>) -> Vec<Arc<dyn Shape>> {
    let mut r: Vec<Arc<dyn Shape>> = Vec::new();
    for i in 0..(mesh.n_indices / 3) {
        r.push(Arc::new(Triangle::new(mesh.clone(), i * 3)));
    }
    r
}

#[derive(Clone, Debug)]
pub struct Triangle {
    mesh: Arc<TriangleMesh>,
    v0: usize,
    v1: usize,
    v2: usize,
}

impl Triangle {
    pub fn new(mesh: Arc<TriangleMesh>, index: usize) -> Self {
        let v0 = mesh.indices[index];
        let v1 = mesh.indices[index + 1];
        let v2 = mesh.indices[index + 2];
        Self {
            mesh,
            v0,
            v1,
            v2,
        }
    }
    pub fn indices(&self) -> (usize, usize, usize) {
        (self.v0, self.v1, self.v2)
    }
    pub fn vertices(&self) -> (Point3f, Point3f, Point3f) {
        let p0 = self.mesh.vertices[self.v0];
        let p1 = self.mesh.vertices[self.v1];
        let p2 = self.mesh.vertices[self.v2];
        (p0, p1, p2)
    }
    pub fn uv(&self) -> (Point2f, Point2f, Point2f) {
        if let Some(uvs) = &self.mesh.uvs {
            (uvs[self.v0], uvs[self.v1], uvs[self.v2])
        } else {
            (
                Point2f::new(0., 0.),
                Point2f::new(1., 0.),
                Point2f::new(1., 1.),
            )
        }
    }
    fn normal(&self) -> Normal3f {
        let (p0, p1, p2) = self.vertices();
        let (dp02, dp12) = (p0 - p2, p1 - p2);
        Normal3f::from(dp02.cross(&dp12).normalize())
    }
    fn uv_interpolate(&self, b0: Float, b1: Float, b2: Float) -> Point2f {
        let (uv0, uv1, uv2) = self.uv();
        Point2f::from(b0 * uv0.coords + b1 * uv1.coords + b2 * uv2.coords)
    }
    fn point_interpolate(&self, b0: Float, b1: Float, b2: Float) -> Point3f {
        let (p0, p1, p2) = self.vertices();
        Point3f::from(b0 * p0.coords + b1 * p1.coords + b2 * p2.coords)
    }
    fn shape_point_interpolate(
        &self,
        b0: Float,
        b1: Float,
        b2: Float,
    ) -> (Point3f, Normal3f, Point2f) {
        (
            self.point_interpolate(b0, b1, b2),
            self.normal(),
            self.uv_interpolate(b0, b1, b2),
        )
    }
    fn abs_sum(&self, b0: Float, b1: Float, b2: Float) -> Vector3f {
        let (p0, p1, p2) = self.vertices();
        (b0 * p0.coords).abs() + (b1 * p1.coords).abs() + (b2 * p2.coords).abs()
    }
}

impl Shape for Triangle {
    fn area(&self) -> Float {
        let (p0, p1, p2) = self.vertices();
        0.5 * (p1 - p0).cross(&(p2 - p0)).magnitude()
    }
    fn bound(&self) -> Bounds3f {
        let (p0, p1, p2) = self.vertices();
        let mut bound = Bounds3f::new(&p0, &p1);
        bound = bound | &p2;
        bound
    }
    fn sample(&self, sampler: &mut dyn Sampler) -> (ShapePoint, Float) {
        let b = uniform_sample_triangle(sampler.get_2d());
        let (b0, b1, b2) = (b.x, b.y, 1. - b.x - b.y);
        let (p, n, uv) = self.shape_point_interpolate(b0, b1, b2);
        let p_error = gamma(6) * self.abs_sum(b0, b1, b2);
        (ShapePoint::new(p, n, uv, p_error), 1. / self.area())
    }
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        let (p0, p1, p2) = self.vertices();
        let o = ray.o;
        let (mut p0t, mut p1t, mut p2t) = (
            Point3f::from(p0 - o),
            Point3f::from(p1 - o),
            Point3f::from(p2 - o),
        );
        let kz = ray.d.abs().imax();
        let kx = if kz + 1 == 3 { 0 } else { kz + 1 };

        let ky = if kx + 1 == 3 { 0 } else { kx + 1 };
        let d = permute(ray.d, kx, ky, kz);
        p0t = permute(p0t, kx, ky, kz);
        p1t = permute(p1t, kx, ky, kz);
        p2t = permute(p2t, kx, ky, kz);
        let sx = -d.x / d.z;
        let sy = -d.y / d.z;
        let sz = 1. / d.z;
        p0t.x += sx * p0t.z;
        p0t.y += sy * p0t.z;
        p1t.x += sx * p1t.z;
        p1t.y += sy * p1t.z;
        p2t.x += sx * p2t.z;
        p2t.y += sy * p2t.z;
        let (e0, e1, e2) = (
            p1t.x * p2t.y - p1t.y * p2t.x,
            p2t.x * p0t.y - p2t.y * p0t.x,
            p0t.x * p1t.y - p0t.y * p1t.x,
        );
        if (e0 < 0. || e1 < 0. || e2 < 0.) && (e0 > 0. || e1 > 0. || e2 > 0.) {
            return false;
        }
        let det = e0 + e1 + e2;
        if det == 0. {
            return false;
        }
        p0t.z *= sz;
        p1t.z *= sz;
        p2t.z *= sz;
        let t_scaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
        if (det < 0. && (t_scaled >= 0. || t_scaled < ray.t_max * det))
            || (det > 0. && (t_scaled <= 0. || t_scaled > ray.t_max * det))
        {
            return false;
        }
        let inv_det = 1. / det;
        let t = t_scaled * inv_det;
        let max_zt = Vector3f::new(p0t.z, p1t.z, p2t.z).abs().max();
        let delta_z = gamma(3) * max_zt;

        let max_xt = Vector3f::new(p0t.x, p1t.x, p2t.x).abs().max();
        let max_yt = Vector3f::new(p0t.y, p1t.y, p2t.y).abs().max();
        let delta_x = gamma(5) * (max_xt + max_zt);
        let delta_y = gamma(5) * (max_yt + max_zt);

        let delta_e = 2. * (gamma(2) * max_xt * max_yt + delta_y * max_xt + delta_x * max_yt);
        let max_e = Vector3f::new(e0, e1, e2).abs().max();

        let delta_t =
            3. * (gamma(3) * max_e * max_zt + delta_e * max_zt + delta_z * max_e) * inv_det.abs();
        if t <= delta_t {
            return false;
        }
        true
    }
    #[allow(clippy::many_single_char_names)]
    fn intersect(&self, ray: &Ray) -> Option<ShapeIntersect> {
        let (p0, p1, p2) = self.vertices();
        let o = ray.o;
        let (mut p0t, mut p1t, mut p2t) = (
            Point3f::from(p0 - o),
            Point3f::from(p1 - o),
            Point3f::from(p2 - o),
        );
        let kz = ray.d.abs().imax();
        let kx = if kz + 1 == 3 { 0 } else { kz + 1 };

        let ky = if kx + 1 == 3 { 0 } else { kx + 1 };
        let d = permute(ray.d, kx, ky, kz);
        p0t = permute(p0t, kx, ky, kz);
        p1t = permute(p1t, kx, ky, kz);
        p2t = permute(p2t, kx, ky, kz);
        let sx = -d.x / d.z;
        let sy = -d.y / d.z;
        let sz = 1. / d.z;
        p0t.x += sx * p0t.z;
        p0t.y += sy * p0t.z;
        p1t.x += sx * p1t.z;
        p1t.y += sy * p1t.z;
        p2t.x += sx * p2t.z;
        p2t.y += sy * p2t.z;
        let (e0, e1, e2) = (
            p1t.x * p2t.y - p1t.y * p2t.x,
            p2t.x * p0t.y - p2t.y * p0t.x,
            p0t.x * p1t.y - p0t.y * p1t.x,
        );
        if (e0 < 0. || e1 < 0. || e2 < 0.) && (e0 > 0. || e1 > 0. || e2 > 0.) {
            return None;
        }
        let det = e0 + e1 + e2;
        if det == 0. {
            return None;
        }
        p0t.z *= sz;
        p1t.z *= sz;
        p2t.z *= sz;
        let t_scaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
        if (det < 0. && (t_scaled >= 0. || t_scaled < ray.t_max * det))
            || (det > 0. && (t_scaled <= 0. || t_scaled > ray.t_max * det))
        {
            return None;
        }
        let inv_det = 1. / det;
        let (b0, b1, b2) = (e0 * inv_det, e1 * inv_det, e2 * inv_det);
        let t = t_scaled * inv_det;
        let max_zt = Vector3f::new(p0t.z, p1t.z, p2t.z).abs().max();
        let delta_z = gamma(3) * max_zt;

        let max_xt = Vector3f::new(p0t.x, p1t.x, p2t.x).abs().max();
        let max_yt = Vector3f::new(p0t.y, p1t.y, p2t.y).abs().max();
        let delta_x = gamma(5) * (max_xt + max_zt);
        let delta_y = gamma(5) * (max_yt + max_zt);

        let delta_e = 2. * (gamma(2) * max_xt * max_yt + delta_y * max_xt + delta_x * max_yt);
        let max_e = Vector3f::new(e0, e1, e2).abs().max();

        let delta_t =
            3. * (gamma(3) * max_e * max_zt + delta_e * max_zt + delta_z * max_e) * inv_det.abs();
        if t <= delta_t {
            return None;
        }
        let p_error = gamma(7) * self.abs_sum(b0, b1, b2);
        let (p, n, uv) = self.shape_point_interpolate(b0, b1, b2);
        Some(ShapeIntersect::new(p, n, t, uv, p_error))
    }
}
