use crate::*;
pub struct OrenNayar {
    r: Spectrum,
    a: Float,
    b: Float,
}

impl OrenNayar {
    pub fn new(r: Spectrum, sigma: Float) -> Self {
        let sigma = sigma.to_radians();
        let sigma2 = sigma * sigma;
        let a = 1. - (sigma2 / (2. * (sigma + 0.33)));
        let b = 0.45 * sigma2 / (sigma2 + 0.09);
        Self { r, a, b }
    }
}

impl BxDF for OrenNayar {
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<Spectrum> {
        let (sin_alpha, tan_beta) = if cos_theta(wi) < cos_theta(wo) {
            // theta_i > theta_o, alpha = theta_i, beta = theta_o
            (sin_theta(wi), tan_theta(wo))
        }
        else {
            // theta_i <= theta_o, alpha = theta_o, beta = theta_i
            (sin_theta(wo), tan_theta(wi))
        };
        Some(self.r * ((self.a + self.b * max(0., cos_delta_phi(wi, wo)) * sin_alpha * tan_beta) / PI))
    }
}
