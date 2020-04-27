use crate::*;

pub trait MicrofacetDistribution {
    fn d(&self, wh: &Vector3f) -> Float;
    fn lambda(&self, w: &Vector3f) -> Float;
    fn g(&self, wo: &Vector3f, wi: &Vector3f) -> Float {
        1. / (1. + self.lambda(wo) + self.lambda(wi))
    }
    fn g1(&self, w: &Vector3f) -> Float {
        1. / (1. + self.lambda(w))
    }
    fn sample_wh(&self, wo: &Vector3f, u: &Point2f) -> (Vector3f, Float);
}

#[derive(Clone, Copy)]
pub struct TrowbridgeReitzDistribution {
    alpha_x: Float,
    alpha_y: Float,
}

impl TrowbridgeReitzDistribution {
    pub fn new(roughu: Float, roughv: Float) -> Self {
        Self {
            alpha_x: roughu,
            alpha_y: roughv,
        }
    }
    pub fn roughness_to_alpha(mut rough: Float) -> Float {
        rough = max(rough, 1e-3);
        let x = rough.ln();
        let x2 = x * x;
        1.62142 + 0.819955 * x + 0.1734 * x2 + 0.0171201 * x * x2 + 0.000640711 * x2 * x2
    }
}

#[allow(clippy::excessive_precision)]
#[allow(non_snake_case)]
impl MicrofacetDistribution for TrowbridgeReitzDistribution {
    fn sample_wh(&self, wo: &Vector3f, u: &Point2f) -> (Vector3f, Float) {
        let flip = wo.z < 0.;
        let wi = if flip { -wo } else { *wo };
        let wi_stretched = Vector3f::new(self.alpha_x * wi.x, self.alpha_y * wi.y, wi.z);
        let cos_theta_i = cos_theta(&wi_stretched);
        let (mut slope_x, mut slope_y) = {
            if cos_theta_i > 0.9999 {
                let r = (u.x / (1. - u.x)).sqrt();
                let phi = 6.28318530718 * u.y;
                (r * phi.cos(), r * phi.sin())
            } else {
                let sin_theta = max(0., 1. - cos_theta_i * cos_theta_i).sqrt();
                let tan_theta = sin_theta / cos_theta_i;
                let a = 1. / tan_theta;
                let G1 = 2. / (1. + (1. + 1. / (a * a)).sqrt());
                let A = 2. * u.x / G1 - 1.;
                let mut tmp = 1. / (A * A - 1.);
                if tmp > 1e10 {
                    tmp = 1e10;
                }
                let B = tan_theta;
                let D = max(B * B * tmp * tmp - (A * A - B * B) * tmp, 0.).sqrt();
                let slope_x_1 = B * tmp - D;
                let slope_x_2 = B * tmp + D;
                let slope_x = if A < 0. || slope_x_2 > 1. / tan_theta {
                    slope_x_1
                } else {
                    slope_x_2
                };

                let (S, u2) = if u.y > 0.5 {
                    (1., 2. * (u.y - 0.5))
                } else {
                    (-1., 2. * (0.5 - u.y))
                };
                let z = (u2 * (u2 * (u2 * 0.27385 - 0.73369) + 0.46341))
                    / (u2 * (u2 * (u2 * 0.093073 + 0.309420) - 1.) + 0.597999);
                let slope_y = S * z * (1. + slope_x * slope_x).sqrt();
                (slope_x, slope_y)
            }
        };
        let tmp = cos_phi(&wi_stretched) * slope_x - sin_phi(&wi_stretched) * slope_y;
        slope_y = sin_phi(&wi_stretched) * slope_x + cos_phi(&wi_stretched) * slope_y;
        slope_x = tmp;

        slope_x *= self.alpha_x;
        slope_y *= self.alpha_y;
        let mut wh = Vector3f::new(-slope_x, -slope_y, 1.).normalize();
        if flip {
            wh = -wh;
        }
        let pdf = self.d(&wh) * self.g1(&wo) * wo.dot(&wh).abs() / cos_theta(&wo).abs();
        (wh, pdf)
    }
    fn d(&self, wh: &Vector3f) -> Float {
        let tan_2_theta = tan_2_theta(wh);
        if tan_2_theta.is_nan() || tan_2_theta.is_infinite() {
            return 0.;
        }
        let cos_4_theta = {
            let cos_2_theta = cos_2_theta(wh);
            cos_2_theta * cos_2_theta
        };
        let e = (cos_2_phi(wh) / (self.alpha_x * self.alpha_x)
            + sin_2_phi(wh) / (self.alpha_y * self.alpha_y))
            * tan_2_theta;
        1. / (PI * self.alpha_x * self.alpha_y * cos_4_theta * (1. + e) * (1. + e))
    }
    fn lambda(&self, w: &Vector3f) -> Float {
        let abs_tan_theta = tan_theta(w).abs();
        if abs_tan_theta.is_nan() || abs_tan_theta.is_infinite() {
            return 0.;
        }
        let alpha = (cos_2_phi(w) * self.alpha_x * self.alpha_x
            + sin_2_phi(w) * self.alpha_y * self.alpha_y)
            .sqrt();
        let alpha_2_tan_2_theta = {
            let alpha_tan_theta = alpha * abs_tan_theta;
            alpha_tan_theta * alpha_tan_theta
        };
        (-1. + (1. + alpha_2_tan_2_theta).sqrt()) / 2.
    }
}

pub struct MicrofacetReflection {
    r: Spectrum,
    distribution: Box<dyn MicrofacetDistribution>,
    fresnel: Box<dyn Fresnel>,
}

impl MicrofacetReflection {
    pub fn new<D: MicrofacetDistribution + 'static, F: Fresnel + 'static>(
        r: Spectrum,
        distribution: D,
        fresnel: F,
    ) -> Self {
        Self {
            r,
            distribution: Box::new(distribution),
            fresnel: Box::new(fresnel),
        }
    }
}

impl BxDF for MicrofacetReflection {
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<RGBSpectrum> {
        let cos_theta_o = cos_theta(wo).abs();
        let cos_theta_i = cos_theta(wi).abs();
        let wi = wi.normalize();
        let wo = wo.normalize();
        let mut wh = wi + wo;
        if (wh.x == 0. && wh.y == 0. && wh.z == 0.) || cos_theta_o == 0. || cos_theta_i == 0. {
            return None;
        }
        wh = wh.normalize();
        let f = self.fresnel.evaluate(wi.dot(&{
            if wh.z < 0. {
                -wh
            } else {
                wh
            }
        }));
        Some(
            self.r * self.distribution.d(&wh) * self.distribution.g(&wo, &wi) * f
                / (4. * cos_theta_o * cos_theta_i),
        )
    }
    fn sample_f(&self, wo: &Vector3f, u: &Point2f) -> (Vector3f, Option<Spectrum>, Float) {
        if wo.z == 0. {
            return (Vector3f::new(0., 0., 0.), None, 0.);
        }
        let (wh, pdf) = self.distribution.sample_wh(wo, u);
        if wo.dot(&wh) < 0. {
            return (Vector3f::new(0., 0., 0.), None, 0.);
        }
        let wi = -wo + 2. * wo.dot(&wh) * wh;
        if wi.z * wo.z < 0. {
            return (Vector3f::new(0., 0., 0.), None, 0.);
        }
        (wi, self.f(wo, &wi), pdf / (4. * wo.dot(&wh)))
    }
}

pub struct MicrofacetTransmission {
    t: Spectrum,
    distribution: Box<dyn MicrofacetDistribution>,
    eta_a: Float,
    eta_b: Float,
    fresnel: FresnelDielectric,
}
impl MicrofacetTransmission {
    pub fn new<D: MicrofacetDistribution + 'static>(
        t: Spectrum,
        distribution: D,
        eta_a: Float,
        eta_b: Float,
    ) -> Self {
        Self {
            t,
            distribution: Box::new(distribution),
            eta_a,
            eta_b,
            fresnel: FresnelDielectric::new(eta_a, eta_b),
        }
    }
}

impl BxDF for MicrofacetTransmission {
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<RGBSpectrum> {
        if wo.z * wi.z > 0. {
            return None;
        }
        let cos_theta_o = cos_theta(wo);
        let cos_theta_i = cos_theta(wi);
        if cos_theta_i == 0. || cos_theta_o == 0. {
            return None;
        }
        let eta = if cos_theta(wo) > 0. {
            self.eta_b / self.eta_a
        } else {
            self.eta_a / self.eta_b
        };
        let mut wh = (wo + wi * eta).normalize();
        if wh.z < 0. {
            wh = -wh;
        }
        let sqrt_denom = wo.dot(&wh) + eta * wi.dot(&wh);
        let f = self.fresnel.evaluate(wo.dot(&wh));
        let factor = 1. / eta;
        Some(
            (Spectrum::new(1.) - f)
                * self.t
                * (self.distribution.d(&wh)
                    * self.distribution.g(wo, wi)
                    * eta
                    * eta
                    * wi.dot(&wh).abs()
                    * wo.dot(&wh).abs()
                    * factor
                    * factor
                    / (cos_theta_i * cos_theta_o * sqrt_denom * sqrt_denom))
                    .abs(),
        )
    }
    fn bxdf_type(&self) -> BxDFType {
        BxDFType::Transmit
    }
    fn sample_f(&self, wo: &Vector3f, u: &Point2f) -> (Vector3f, Option<Spectrum>, Float) {
        if wo.z == 0. {
            return (Vector3f::new(0., 0., 0.), None, 0.);
        }
        let (wh, pdf) = self.distribution.sample_wh(wo, u);
        if wo.dot(&wh) < 0. {
            return (Vector3f::new(0., 0., 0.), None, 0.);
        }
        let (eta_i, eta_o)  = if cos_theta(wo) > 0. {
            (self.eta_a / self.eta_b, self.eta_b / self.eta_a)
        } else {
            (self.eta_b / self.eta_a, self.eta_a / self.eta_b)
        };
        if let Some(wi) = refract(wo, &wh.into(), eta_i) {
            let sqrt_denom = wo.dot(&wh) + eta_o * wi.dot(&wh);
            let dwh_dwi = (eta_o * eta_o * wi.dot(&wh)).abs() / (sqrt_denom * sqrt_denom);
            (wi, self.f(wo, &wi), pdf * dwh_dwi)
        } else {
            (Vector3f::new(0., 0., 0.), None, 0.)
        }
    }
}
