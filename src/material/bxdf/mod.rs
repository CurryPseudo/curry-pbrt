mod lambertian;
mod oren_nayar;
mod specular;
use crate::*;
pub use lambertian::*;
pub use oren_nayar::*;
pub use specular::*;
use std::{
    ops::{Add, AddAssign, Div, DivAssign},
    sync::Arc,
};

pub trait BxDF {
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<Spectrum>;
    fn sample_f(
        &self,
        wo: &Vector3f,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float) {
        let (wi, pdf) = cosine_sample_hemisphere(sampler.get_2d());
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
    fn average<T: AddAssign + DivAssign<Float> + Default, F: Fn(&dyn BxDF) -> T>(&self, f: F) -> T {
        let sum = T::default();
        self.average_general(|bxdf, sum| *sum += f(bxdf), |sum, f| *sum /= f, sum)
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
        w.x * self.snx + w.y * self.sny + w.z * self.sn
    }
    fn world_to_local(&self, w: &Vector3f) -> Vector3f {
        Vector3f::new(w.dot(&self.snx), w.dot(&self.sny), w.dot(&self.sn))
    }
    pub fn add_bxdf<T: BxDF + 'static>(&mut self, bxdf: Arc<T>) {
        self.bxdfs.push(bxdf);
    }
    pub fn add_delta_bxdf<T: DeltaBxDF + BxDF + 'static>(&mut self, delta_bxdf: Arc<T>) {
        self.delta_bxdfs.push(delta_bxdf.clone());
        self.bxdfs.push(delta_bxdf);
    }
    pub fn sample_delta_wi(&self, wo: &Vector3f) -> Vec<(Vector3f, Spectrum)> {
        let mut r = Vec::new();
        for delta_bxdf in &self.delta_bxdfs {
            let (wi, s) = delta_bxdf.sample_f(&self.world_to_local(wo));
            if let Some(s) = s {
                r.push((self.local_to_world(&wi), s));
            }
        }
        r
    }
}

impl BxDF for BSDF {
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<Spectrum> {
        if wo.dot(&self.n) * wi.dot(&self.n) <= 0. {
            None
        } else {
            let wo = self.world_to_local(wo);
            let wi = self.world_to_local(wi);
            let s: Spectrum = self.average(|bxdf| bxdf.f(&wo, &wi).into());
            s.to_option()
        }
    }
    fn pdf(&self, wo: &Vector3f, wi: &Vector3f) -> Float {
        let wo = self.world_to_local(wo);
        let wi = self.world_to_local(wi);
        let pdf = self.average(|bxdf| bxdf.pdf(&wo, &wi));
        pdf
    }
    fn f_pdf(&self, wo: &Vector3f, wi: &Vector3f) -> (Option<Spectrum>, Float) {
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
    fn sample_f(
        &self,
        wo: &Vector3f,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float) {
        let choose_index = sampler.get_usize(self.bxdfs.len());
        let choose_bxdf = &self.bxdfs[choose_index];
        let wo_local = self.world_to_local(wo);
        let (wi_local, f, mut pdf) = choose_bxdf.sample_f(&wo_local, sampler);
        let mut f: Spectrum = f.into();
        for i in 0..self.bxdfs.len() {
            if i != choose_index {
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
}

pub trait DeltaBxDF {
    fn sample_f(&self, wo: &Vector3f) -> (Vector3f, Option<Spectrum>);
}

impl<T: DeltaBxDF> BxDF for T {
    fn sample_f(
        &self,
        wo: &Vector3f,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float) {
        sampler.get_2d();
        let (wi, s) = self.sample_f(wo);
        (wi, s, 1.)
    }
    fn pdf(&self, _wo: &Vector3f, _wi: &Vector3f) -> Float {
        0.
    }
    fn f(&self, _wo: &Vector3f, _wi: &Vector3f) -> Option<Spectrum> {
        None
    }
}
