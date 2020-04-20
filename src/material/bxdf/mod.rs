mod lambertian;
mod oren_nayar;
mod specular;
use crate::*;
pub use lambertian::*;
pub use oren_nayar::*;
pub use specular::*;
use std::sync::Arc;

pub trait BxDF {
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<Spectrum>;
    fn sample_f(&self, wo: &Vector3f, u: &Point2f) -> (Vector3f, Option<Spectrum>, Float) {
        let (mut wi, pdf) = cosine_sample_hemisphere(*u);
        if wo.z < 0. {
            wi.z *= -1.;
        }
        let f = self.f(wo, &wi);
        (wi, f, pdf)
    }
    fn pdf(&self, _wo: &Vector3f, wi: &Vector3f) -> Float {
        wi.z * INV_PI
    }
    fn f_pdf(&self, wo: &Vector3f, wi: &Vector3f) -> (Option<Spectrum>, Float) {
        (self.f(wo, wi), self.pdf(wo, wi))
    }
    fn cos_theta(&self, w: &Vector3f) -> Float {
        w.z
    }
    fn cos_2_theta(&self, w: &Vector3f) -> Float {
        w.z * w.z
    }
    fn sin_2_theta(&self, w: &Vector3f) -> Float {
        1. - self.cos_2_theta(w)
    }
    fn sin_theta(&self, w: &Vector3f) -> Float {
        self.sin_2_theta(w).sqrt()
    }
    fn tan_theta(&self, w: &Vector3f) -> Float {
        self.sin_theta(w) / self.cos_theta(w)
    }
    fn tan_2_theta(&self, w: &Vector3f) -> Float {
        self.sin_2_theta(w) / self.cos_2_theta(w)
    }
    fn cos_phi(&self, w: &Vector3f) -> Float {
        let sin_theta = self.sin_theta(w);
        if sin_theta == 0. {
            1.
        } else {
            clamp(w.x / sin_theta, -1., 1.)
        }
    }
    fn sin_phi(&self, w: &Vector3f) -> Float {
        let sin_theta = self.sin_theta(w);
        if sin_theta == 0. {
            1.
        } else {
            clamp(w.y / sin_theta, -1., 1.)
        }
    }
    fn cos_2_phi(&self, w: &Vector3f) -> Float {
        let cos_phi = self.cos_phi(w);
        cos_phi * cos_phi
    }
    fn sin_2_phi(&self, w: &Vector3f) -> Float {
        let sin_phi = self.sin_phi(w);
        sin_phi * sin_phi
    }
    fn cos_delta_phi(&self, wa: &Vector3f, wb: &Vector3f) -> Float {
        clamp(
            (wa.x * wb.x + wa.y * wb.y)
                / ((wa.x * wa.x + wa.y * wa.y) * (wb.x * wb.x + wb.y * wb.y)).sqrt(),
            -1.,
            1.,
        )
    }
}

pub struct BSDF {
    n: Vector3f,
    sn: Vector3f,
    snx: Vector3f,
    sny: Vector3f,
    bxdfs: Vec<Arc<dyn BxDF>>,
    delta_bxdfs: Vec<Arc<dyn DeltaBxDF>>,
}

impl BSDF {
    pub fn new(n: Normal3f, sn: Normal3f) -> Self {
        let sn = sn.into();
        let (snx, sny) = coordinate_system(&sn);
        let n = n.x * snx + n.y * sny + n.z * sn;
        Self {
            n,
            sn,
            snx,
            sny,
            bxdfs: Vec::new(),
            delta_bxdfs: Vec::new(),
        }
    }
    fn average_general<A: Fn(&dyn BxDF, &mut T), D: Fn(&mut T, Float), T>(
        &self,
        a: A,
        d: D,
        mut t: T,
    ) -> T {
        for bxdf in &self.bxdfs {
            a(bxdf.as_ref(), &mut t);
        }
        d(&mut t, self.bxdfs.len() as Float);
        t
    }
    fn local_to_world(&self, w: &Vector3f) -> Vector3f {
        let snx = &self.snx;
        let sny = &self.sny;
        let sn = &self.sn;
        Vector3f::new(
            snx.x * w.x + sny.x * w.y + sn.x * w.z,
            snx.y * w.x + sny.y * w.y + sn.y * w.z,
            snx.z * w.x + sny.z * w.y + sn.z * w.z,
        )
    }
    fn world_to_local(&self, w: &Vector3f) -> Vector3f {
        Vector3f::new(w.dot(&self.snx), w.dot(&self.sny), w.dot(&self.sn))
    }
    pub fn add_bxdf<T: BxDF + 'static>(&mut self, bxdf: Arc<T>) {
        self.bxdfs.push(bxdf);
    }
    pub fn add_delta_bxdf<T: DeltaBxDF + BxDF + 'static>(&mut self, delta_bxdf: Arc<T>) {
        self.delta_bxdfs.push(delta_bxdf.clone());
    }
    pub fn sample_all_delta_f(&self, wo: &Vector3f) -> Vec<(Vector3f, Spectrum)> {
        let mut r = Vec::new();
        for delta_bxdf in &self.delta_bxdfs {
            if let Some((wi, s)) = delta_bxdf.sample_f(&self.world_to_local(wo)) {
                r.push((self.local_to_world(&wi), s));
            }
        }
        r
    }
    fn choose_no_delta_f(
        &self,
        index: usize,
        wo: &Vector3f,
        u: &Point2f,
    ) -> (Vector3f, Option<Spectrum>, Float) {
        let choose_bxdf = &self.bxdfs[index];
        let wo_local = self.world_to_local(wo);
        let (wi_local, f, mut pdf) = choose_bxdf.sample_f(&wo_local, u);
        let mut f: Spectrum = f.into();
        for i in 0..self.bxdfs.len() {
            if i != index {
                let (bxdf_f, bxdf_pdf) = self.bxdfs[i].f_pdf(&wo_local, &wi_local);
                f += bxdf_f;
                pdf += bxdf_pdf;
            }
        }
        let bxdfs_len_f = self.bxdfs.len() as Float;
        f /= bxdfs_len_f;
        pdf /= bxdfs_len_f;
        (self.local_to_world(&wi_local), f.to_option(), pdf)
    }
    pub fn sample_no_delta_f(
        &self,
        wo: &Vector3f,
        u: &Point2f,
    ) -> (Vector3f, Option<Spectrum>, Float) {
        if self.bxdfs.is_empty() {
            return (Vector3f::new(0., 0., 0.), None, 0.);
        }
        let (index, remap) = sample_usize_remap(u.x, self.bxdfs.len());
        self.choose_no_delta_f(index, wo, &Point2f::new(remap, u.y))
    }
    pub fn sample_delta_f(&self, wo: &Vector3f, u: Float) -> (Vector3f, Option<Spectrum>, Float) {
        let wo_local = &self.world_to_local(wo);
        if self.delta_bxdfs.is_empty() {
            return (Vector3f::new(0., 0., 0.), None, 0.);
        }
        let mut wi_f = Vec::new();
        for i in 0..self.delta_bxdfs.len() {
            let delta_bxdf = &self.delta_bxdfs[i];
            if let Some((wi_local, f)) = delta_bxdf.sample_f(wo_local) {
                wi_f.push((wi_local, f));
            }
        }
        let (i, pdf, _) = sample_distribution_1d_remap(u, wi_f.len(), &|i| wi_f[i].1.y());
        let (wi_local, f) = wi_f[i];
        (self.local_to_world(&wi_local), Some(f), pdf)
    }
    pub fn f_pdf(&self, wo: &Vector3f, wi: &Vector3f) -> (Option<Spectrum>, Float) {
        if self.bxdfs.is_empty() {
            return (None, 0.);
        }
        let wo = self.world_to_local(wo);
        let wi = self.world_to_local(wi);
        let (f, pdf) = self.average_general(
            |bxdf, (f, pdf)| {
                let (this_f, this_pdf) = bxdf.f_pdf(&wo, &wi);
                *f += this_f;
                *pdf += this_pdf
            },
            |(f, pdf), len| {
                *f /= len;
                *pdf /= len;
            },
            (Spectrum::default(), 0.),
        );
        (f.to_option(), pdf)
    }
    pub fn sample_f(
        &self,
        wo: &Vector3f,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float, bool) {
        let (i, pdf, remap) = sampler.get_distribution_1d_remap(2, &|i| if i == 0 {self.bxdfs.len()} else {self.delta_bxdfs.len()} as Float );
        let ((wi, f, internal_pdf), is_delta) = if i == 0 {
            (
                self.sample_no_delta_f(wo, &Point2f::new(remap, sampler.get_1d())),
                false,
            )
        } else {
            (self.sample_delta_f(wo, remap), true)
        };
        (wi, f, pdf * internal_pdf, is_delta)
    }
    pub fn is_all_delta(&self) -> bool {
        self.bxdfs.is_empty()
    }
}

pub trait DeltaBxDF {
    fn sample_f(&self, wo: &Vector3f) -> Option<(Vector3f, Spectrum)>;
}

impl<T: DeltaBxDF> BxDF for T {
    fn sample_f(&self, wo: &Vector3f, _: &Point2f) -> (Vector3f, Option<Spectrum>, Float) {
        if let Some((wi, s)) = self.sample_f(wo) {
            (wi, Some(s), 1.)
        } else {
            (Vector3f::new(0., 0., 0.), None, 0.)
        }
    }
    fn pdf(&self, _wo: &Vector3f, _wi: &Vector3f) -> Float {
        0.
    }
    fn f(&self, _wo: &Vector3f, _wi: &Vector3f) -> Option<Spectrum> {
        None
    }
}
