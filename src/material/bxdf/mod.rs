mod lambertian;
mod microfacet;
mod oren_nayar;
mod specular;
use crate::*;
pub use lambertian::*;
pub use microfacet::*;
pub use oren_nayar::*;
pub use specular::*;
use std::sync::Arc;

pub enum BxDFType {
    Delta,
    Reflect,
    Transmit,
}

pub trait BxDF {
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<Spectrum>;
    fn sample_f(&self, wo: &Vector3f, u: &Point2f) -> (Vector3f, Option<Spectrum>, Float) {
        let (mut wi, pdf) = cosine_sample_hemisphere(*u);
        match self.bxdf_type() {
            BxDFType::Reflect => {
                if wo.z < 0. {
                    wi.z *= -1.;
                }
            }
            BxDFType::Transmit => {
                if wo.z > 0. {
                    wi.z *= -1.;
                }
            }
            _ => (),
        };
        let f = self.f(wo, &wi);
        (wi, f, pdf)
    }
    fn pdf(&self, _wo: &Vector3f, wi: &Vector3f) -> Float {
        wi.z * INV_PI
    }
    fn f_pdf(&self, wo: &Vector3f, wi: &Vector3f) -> (Option<Spectrum>, Float) {
        (self.f(wo, wi), self.pdf(wo, wi))
    }
    fn bxdf_type(&self) -> BxDFType {
        BxDFType::Reflect
    }
}

pub struct ScaledBxDF(Arc<dyn BxDF>, Spectrum);

impl BxDF for ScaledBxDF {
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<Spectrum> {
        self.0.f(wo, wi).map(|f| f * self.1)
    }
    fn sample_f(&self, wo: &Vector3f, u: &Point2f) -> (Vector3f, Option<Spectrum>, Float) {
        let (wi, s, pdf) = self.0.sample_f(wo, u);
        (wi, s.map(|f| f * self.1), pdf)
    }
    fn pdf(&self, wo: &Vector3f, wi: &Vector3f) -> Float {
        self.0.pdf(wo, wi)
    }
    fn f_pdf(&self, wo: &Vector3f, wi: &Vector3f) -> (Option<Spectrum>, Float) {
        let (f, pdf) = self.0.f_pdf(wo, wi);
        (f.map(|f| f * self.1), pdf)
    }
    fn bxdf_type(&self) -> BxDFType {
        self.0.bxdf_type()
    }
}

pub struct BSDF {
    n: Vector3f,
    sn: Vector3f,
    snx: Vector3f,
    sny: Vector3f,
    bxdfs: Vec<Arc<dyn BxDF>>,
    reflect_bxdfs: Vec<Arc<dyn BxDF>>,
    transmit_bxdfs: Vec<Arc<dyn BxDF>>,
    delta_bxdfs: Vec<Arc<dyn DeltaBxDF>>,
}

impl BSDF {
    pub fn new(n: Normal3f, sn: Normal3f) -> Self {
        let sn = sn.into();
        let (snx, sny) = coordinate_system(&sn);
        let n = Vector3f::new(n.dot(&snx), n.dot(&sny), n.dot(&sn)).normalize();
        Self {
            n,
            sn,
            snx,
            sny,
            bxdfs: Vec::new(),
            reflect_bxdfs: Vec::new(),
            transmit_bxdfs: Vec::new(),
            delta_bxdfs: Vec::new(),
        }
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
        .normalize()
    }
    fn world_to_local(&self, w: &Vector3f) -> Vector3f {
        Vector3f::new(w.dot(&self.snx), w.dot(&self.sny), w.dot(&self.sn)).normalize()
    }
    pub fn add_bxdf<T: BxDF + 'static>(&mut self, bxdf: Arc<T>) {
        self.bxdfs.push(bxdf.clone());
        match bxdf.bxdf_type() {
            BxDFType::Reflect => {
                self.reflect_bxdfs.push(bxdf);
            }
            BxDFType::Transmit => {
                self.transmit_bxdfs.push(bxdf);
            }
            BxDFType::Delta => panic!(),
        }
    }
    pub fn add_delta_bxdf<T: DeltaBxDF + BxDF + 'static>(&mut self, delta_bxdf: Arc<T>) {
        self.delta_bxdfs.push(delta_bxdf);
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
        let (wi_local, f, pdf) = choose_bxdf.sample_f(&wo_local, u);
        let wi = self.local_to_world(&wi_local);
        let bxdfs_len_f = self.bxdfs.len() as Float;
        (wi, f, pdf / bxdfs_len_f)
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
    pub fn no_delta_f_pdf(&self, wo: &Vector3f, wi: &Vector3f) -> (Option<Spectrum>, Float) {
        if self.bxdfs.is_empty() {
            return (None, 0.);
        }
        let wo = self.world_to_local(wo);
        let wi = self.world_to_local(wi);
        let reflect = wo.dot(&self.n) * wi.dot(&self.n) > 0.;
        let mut f = None;
        let mut pdf = 0.;
        let bxdfs = if reflect {
            &self.reflect_bxdfs
        } else {
            &self.transmit_bxdfs
        };
        for bxdf in bxdfs {
            if let (Some(this_f), this_pdf) = bxdf.f_pdf(&wo, &wi) {
                f = Some(f.unwrap_or_else(|| Spectrum::new(0.)) + this_f);
                pdf += this_pdf;
            }
        }
        let len = bxdfs.len() as Float;
        (f, pdf / len)
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
    pub fn mix(self, rhs: Self, scale: Spectrum) -> Self {
        let s1 = scale;
        let s2 = Spectrum::new(1.) - s1;
        if s1.is_black() {
            return rhs;
        }
        if s2.is_black() {
            return self;
        }
        let mut bxdfs: Vec<Arc<dyn BxDF>> = Vec::new();
        for bxdf in self.bxdfs {
            bxdfs.push(Arc::new(ScaledBxDF(bxdf, s1)));
        }
        for bxdf in rhs.bxdfs {
            bxdfs.push(Arc::new(ScaledBxDF(bxdf, s2)));
        }
        let mut reflect_bxdfs: Vec<Arc<dyn BxDF>> = Vec::new();
        for bxdf in self.reflect_bxdfs {
            reflect_bxdfs.push(Arc::new(ScaledBxDF(bxdf, s1)));
        }
        for bxdf in rhs.reflect_bxdfs {
            reflect_bxdfs.push(Arc::new(ScaledBxDF(bxdf, s2)));
        }
        let mut transmit_bxdfs: Vec<Arc<dyn BxDF>> = Vec::new();
        for bxdf in self.transmit_bxdfs {
            transmit_bxdfs.push(Arc::new(ScaledBxDF(bxdf, s1)));
        }
        for bxdf in rhs.transmit_bxdfs {
            transmit_bxdfs.push(Arc::new(ScaledBxDF(bxdf, s2)));
        }
        let mut delta_bxdfs: Vec<Arc<dyn DeltaBxDF>> = Vec::new();
        for bxdf in self.delta_bxdfs {
            delta_bxdfs.push(Arc::new(ScaledDeltaBxDF(bxdf, s1)));
        }
        for bxdf in rhs.delta_bxdfs {
            delta_bxdfs.push(Arc::new(ScaledDeltaBxDF(bxdf, s2)));
        }
        let n = self.n;
        let sn = self.sn;
        let snx = self.snx;
        let sny = self.sny;
        Self {
            n,
            sn,
            snx,
            sny,
            bxdfs,
            reflect_bxdfs,
            transmit_bxdfs,
            delta_bxdfs,
        }
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
    fn bxdf_type(&self) -> BxDFType {
        BxDFType::Delta
    }
}
pub struct ScaledDeltaBxDF(Arc<dyn DeltaBxDF>, Spectrum);

impl DeltaBxDF for ScaledDeltaBxDF {
    fn sample_f(&self, wo: &Vector3f) -> Option<(Vector3f, Spectrum)> {
        self.0.sample_f(wo).map(|(wi, f)| (wi, f * self.1))
    }
}
