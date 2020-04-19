use crate::*;
pub trait Fresnel {
    fn evaluate(&self, cos_i: Float) -> Float;
}
pub struct FresnelDielectric {
    eta_i: Float,
    eta_t: Float,
}

impl FresnelDielectric {
    pub fn new(eta_i: Float, eta_t: Float) -> Self {
        Self { eta_i, eta_t }
    }
}

impl Fresnel for FresnelDielectric {
    fn evaluate(&self, mut cos_i: Float) -> Float {
        let (eta_i, eta_t) = if cos_i > 0. {
            (self.eta_i, self.eta_t)
        } else {
            (self.eta_t, self.eta_i)
        };
        cos_i = cos_i.abs();
        let sin_i = (1. - cos_i * cos_i).sqrt();
        let sin_t = sin_i * eta_i / eta_t;
        if sin_t >= 1. {
            return 1.;
        }
        let cos_t = (1. - sin_t * sin_t).sqrt();
        let (t0, t1) = (eta_t * cos_i, eta_i * cos_t);
        let r_perl = (t0 - t1) / (t0 + t1);
        let (t0, t1) = (eta_i * cos_i, eta_t * cos_t);
        let r_perp = (t0 - t1) / (t0 + t1);
        (r_perl * r_perl + r_perp * r_perp) / 2.
    }
}
pub struct SpecularReflection {
    r: Spectrum,
    fresnel: Box<dyn Fresnel>,
}

impl SpecularReflection {
    pub fn new(r: Spectrum, fresnel: Box<dyn Fresnel>) -> Self {
        Self { r, fresnel }
    }
}

impl DeltaBxDF for SpecularReflection {
    fn sample_f(&self, wo: &Vector3f) -> Option<(Vector3f, Spectrum)> {
        let wi = Vector3f::new(-wo.x, -wo.y, wo.z);
        let cos_theta_i = self.cos_theta(&wi);
        let fresnel_r = self.fresnel.evaluate(cos_theta_i);
        if fresnel_r == 0. {
            return None;
        }
        let s = self.r * fresnel_r / cos_theta_i.abs();
        Some((wi, s))
    }
}

pub struct SpecularTransmission {
    t: Spectrum,
    eta_a: Float,
    eta_b: Float,
    fresnel: FresnelDielectric,
}

impl SpecularTransmission {
    pub fn new(t: Spectrum, eta_a: Float, eta_b: Float) -> Self {
        Self {
            t,
            eta_a,
            eta_b,
            fresnel: FresnelDielectric::new(eta_a, eta_b),
        }
    }
}

impl DeltaBxDF for SpecularTransmission {
    fn sample_f(&self, wo: &Vector3f) -> Option<(Vector3f, Spectrum)> {
        let cos_theta_o = self.cos_theta(wo);
        let (eta_i, eta_t) = if self.cos_theta(wo) > 0. {
            (self.eta_a, self.eta_b)
        } else {
            (self.eta_b, self.eta_a)
        };
        let eta = eta_i / eta_t;
        let n = Vector3f::new(0., 0., cos_theta_o.signum());
        let cos_theta_o = cos_theta_o.abs();
        let sin_2_theta_o = 1. - cos_theta_o * cos_theta_o;
        let sin_2_theta_i = sin_2_theta_o * eta * eta;
        if sin_2_theta_i > 1. {
            return None;
        }
        let cos_theta_i = (1. - sin_2_theta_i).sqrt();
        let wi = eta * (-wo) + (eta * cos_theta_o - cos_theta_i) * n;
        let fresnel_t = 1. - self.fresnel.evaluate(self.cos_theta(&wi));
        if fresnel_t == 0. {
            None
        } else {
            Some((wi, self.t * fresnel_t / self.cos_theta(&wi).abs()))
        }
    }
}
