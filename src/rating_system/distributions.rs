use core::ops;

use statrs::function::erf::erfc;
use std::f64::consts::PI;

const PREC: f64 = 1e-3;

#[derive(Clone, Debug)]
pub struct Gaussian {
    pub mu: f64,
    pub sigma: f64,
}

impl ops::Add<Gaussian> for Gaussian {
    type Output = Gaussian;

    fn add(self, to: Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu + to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl<'a> ops::Add<Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn add(self, to: Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu + to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}

impl<'b> ops::Add<&'b Gaussian> for Gaussian {
    type Output = Gaussian;

    fn add(self, to: &'b Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu + to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl<'a, 'b> ops::Add<&'b Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn add(self, to: &'b Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu + to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl ops::Sub<Gaussian> for Gaussian {
    type Output = Gaussian;

    fn sub(self, to: Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu - to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl<'a> ops::Sub<Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn sub(self, to: Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu - to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl<'b> ops::Sub<&'b Gaussian> for Gaussian {
    type Output = Gaussian;

    fn sub(self, to: &'b Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu - to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl<'a, 'b> ops::Sub<&'b Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn sub(self, to: &'b Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu - to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl ops::AddAssign<Gaussian> for Gaussian {
    fn add_assign(&mut self, to: Gaussian) {
        *self = Self {
            mu: self.mu + to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        };
    }
}


impl<'b> ops::AddAssign<&'b Gaussian> for Gaussian {
    fn add_assign(&mut self, to: &'b Gaussian) {
        *self = Self {
            mu: self.mu + to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        };
    }
}


impl ops::SubAssign<Gaussian> for Gaussian {
    fn sub_assign(&mut self, to: Gaussian) {
        *self = Self {
            mu: self.mu - to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        };
    }
}


impl<'b> ops::SubAssign<&'b Gaussian> for Gaussian {
    fn sub_assign(&mut self, to: &'b Gaussian) {
        *self = Self {
            mu: self.mu - to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        };
    }
}


impl ops::Mul<f64> for Gaussian {
    type Output = Gaussian;

    fn mul(self, to: f64) -> Gaussian {
        Gaussian {
            mu: self.mu * to,
            sigma: self.sigma * to.abs(),
        }
    }
}

impl<'b> ops::Mul<&'b f64> for Gaussian {
    type Output = Gaussian;

    fn mul(self, to: &'b f64) -> Gaussian {
        Gaussian {
            mu: self.mu * *to,
            sigma: self.sigma * to.abs(),
        }
    }
}


impl<'a> ops::Mul<f64> for &'a Gaussian {
    type Output = Gaussian;

    fn mul(self, to: f64) -> Gaussian {
        Gaussian {
            mu: self.mu * to,
            sigma: self.sigma * to.abs(),
        }
    }
}

impl<'a, 'b> ops::Mul<&'b f64> for &'a Gaussian {
    type Output = Gaussian;

    fn mul(self, to: &'b f64) -> Gaussian {
        Gaussian {
            mu: self.mu * *to,
            sigma: self.sigma * to.abs(),
        }
    }
}

impl ops::Mul<Gaussian> for Gaussian {
    type Output = Gaussian;

    fn mul(self, to: Gaussian) -> Gaussian {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);
        Gaussian {
            mu: (self.mu * ssigma2 + to.mu * ssigma1) / (ssigma1 + ssigma2),
            sigma: self.sigma * to.sigma / (ssigma1 + ssigma2).sqrt(),
        }
    }
}

impl<'a> ops::Mul<Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn mul(self, to: Gaussian) -> Gaussian {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);
        Gaussian {
            mu: (self.mu * ssigma2 + to.mu * ssigma1) / (ssigma1 + ssigma2),
            sigma: self.sigma * to.sigma / (ssigma1 + ssigma2).sqrt(),
        }
    }
}

impl<'b> ops::Mul<&'b Gaussian> for Gaussian {
    type Output = Gaussian;

    fn mul(self, to: &'b Gaussian) -> Gaussian {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);
        Gaussian {
            mu: (self.mu * ssigma2 + to.mu * ssigma1) / (ssigma1 + ssigma2),
            sigma: self.sigma * to.sigma / (ssigma1 + ssigma2).sqrt(),
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn mul(self, to: &'b Gaussian) -> Gaussian {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);
        Gaussian {
            mu: (self.mu * ssigma2 + to.mu * ssigma1) / (ssigma1 + ssigma2),
            sigma: self.sigma * to.sigma / (ssigma1 + ssigma2).sqrt(),
        }
    }
}

impl ops::MulAssign<Gaussian> for Gaussian {
    fn mul_assign(&mut self, to: Gaussian) {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);

        *self = Gaussian {
            mu: (self.mu * ssigma2 + to.mu * ssigma1) / (ssigma1 + ssigma2),
            sigma: self.sigma * to.sigma / (ssigma1 + ssigma2).sqrt(),
        };
    }
}

impl<'b> ops::MulAssign<&'b Gaussian> for Gaussian {
    fn mul_assign(&mut self, to: &'b Gaussian) {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);

        *self = Gaussian {
            mu: (self.mu * ssigma2 + to.mu * ssigma1) / (ssigma1 + ssigma2),
            sigma: self.sigma * to.sigma / (ssigma1 + ssigma2).sqrt(),
        };
    }
}


impl ops::Div<Gaussian> for Gaussian {
    type Output = Gaussian;

    fn div(self, to: Gaussian) -> Gaussian {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);
        Gaussian {
            mu: (self.mu * ssigma2 - to.mu * ssigma1) / (ssigma2 - ssigma1),
            sigma: self.sigma * to.sigma / (ssigma2 - ssigma1).abs().sqrt(),
        }
    }
}


impl<'a> ops::Div<Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn div(self, to: Gaussian) -> Gaussian {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);
        Gaussian {
            mu: (self.mu * ssigma2 - to.mu * ssigma1) / (ssigma2 - ssigma1),
            sigma: self.sigma * to.sigma / (ssigma2 - ssigma1).abs().sqrt(),
        }
    }
}

impl<'b> ops::Div<&'b Gaussian> for Gaussian {
    type Output = Gaussian;

    fn div(self, to: &'b Gaussian) -> Gaussian {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);
        Gaussian {
            mu: (self.mu * ssigma2 - to.mu * ssigma1) / (ssigma2 - ssigma1),
            sigma: self.sigma * to.sigma / (ssigma2 - ssigma1).abs().sqrt(),
        }
    }
}


impl<'a, 'b> ops::Div<&'b Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn div(self, to: &'b Gaussian) -> Gaussian {
        let ssigma1 = self.sigma.powi(2);
        let ssigma2 = to.sigma.powi(2);
        Gaussian {
            mu: (self.mu * ssigma2 - to.mu * ssigma1) / (ssigma2 - ssigma1),
            sigma: self.sigma * to.sigma / (ssigma2 - ssigma1).abs().sqrt(),
        }
    }
}

fn gauss_exponent(mu: f64, sigma: f64, t: f64) -> f64 {
    (-((t - mu) / sigma).powi(2)).exp()
}


fn moment0(mu: f64, sigma: f64, t: f64) -> f64 {
    sigma * PI.sqrt() / 2. * erfc((t - mu) / sigma)
}


fn moment1(mu: f64, sigma: f64, t: f64) -> f64 {
    mu * moment0(0., sigma, t - mu) + sigma.powi(2) / 2. * gauss_exponent(mu, sigma, t)
}


fn moment2(mu: f64, sigma: f64, t: f64) -> f64 {
    mu.powi(2) * moment0(0., sigma, t - mu) + 2. * mu * moment1(0., sigma, t - mu) +
        sigma.powi(2) / 4. * (2. * gauss_exponent(mu, sigma, t) * (t - mu) +
            sigma * PI.sqrt() * erfc((t - mu) / sigma))
}


impl Gaussian {
    pub fn leq_eps(&self, eps: f64) -> Gaussian {
        assert!(eps >= 0.);

        let alpha = moment0(self.mu, self.sigma, -eps) - moment0(self.mu, self.sigma, eps);

        if alpha < PREC {
            return Gaussian { mu: 0., sigma: (1. / 3. as f64).sqrt() } / self;
        }

        let mu = 1. / alpha * (moment1(self.mu, self.sigma, -eps) - moment1(self.mu, self.sigma, eps));
        let sigma2 = 1. / alpha * (moment2(self.mu, self.sigma, -eps) - moment2(self.mu, self.sigma, eps)) - mu.powi(2);
        let sigma = sigma2.sqrt();

        if !(!mu.is_nan() && !sigma.is_nan()) {
            println!("{:?}\teps {}", self, eps);
            panic!();
        }

        Gaussian { mu, sigma } / self
    }


    pub fn greater_eps(&self, eps: f64) -> Gaussian {
        assert!(eps >= 0.);

        let alpha = moment0(self.mu, self.sigma, eps);

        if alpha < PREC {
            return Gaussian { mu: eps, sigma: self.sigma / (2. as f64).sqrt() } / self;
        }

        let mu = 1. / alpha * moment1(self.mu, self.sigma, eps);
        let sigma2 = 1. / alpha * moment2(self.mu, self.sigma, eps) - mu.powi(2);
        let sigma = sigma2.sqrt();

        if !(!mu.is_nan() && !sigma.is_nan()) {
            println!("{:?}\teps {}", self, eps);
            panic!();
        }

        Gaussian { mu, sigma } / self
    }
}
