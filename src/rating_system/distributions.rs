use core::ops;

use statrs::function::erf::erf;
use std::f64::consts::PI;

#[derive(Clone)]
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
            mu: self.mu + to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl<'a> ops::Sub<Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn sub(self, to: Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu + to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl<'b> ops::Sub<&'b Gaussian> for Gaussian {
    type Output = Gaussian;

    fn sub(self, to: &'b Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu + to.mu,
            sigma: (self.sigma.powi(2) + to.sigma.powi(2)).sqrt(),
        }
    }
}


impl<'a, 'b> ops::Sub<&'b Gaussian> for &'a Gaussian {
    type Output = Gaussian;

    fn sub(self, to: &'b Gaussian) -> Gaussian {
        Gaussian {
            mu: self.mu + to.mu,
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
            mu: self.mu * to,
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

fn normal_exp(gaussian: &Gaussian, eps: f64) -> f64 {
    (-((eps + gaussian.mu).powi(2)) / (2. * gaussian.sigma.powi(2))).exp()
}

impl Gaussian {
    pub fn leq_eps(&self, eps: f64) -> Gaussian {
        let alpha = 0.5 * erf((eps + self.mu) / ((2. as f64).sqrt() * self.sigma))
            - 0.5 * erf((-eps + self.mu) / ((2. as f64).sqrt() * self.sigma));
        let mu = self.mu + self.sigma
            * (normal_exp(self, -eps) + normal_exp(self, eps))
            / (alpha * (2. * PI).sqrt());
        let sigma = self.mu.powi(2) + self.sigma.powi(2) - self.sigma
            * ((eps + self.mu) * normal_exp(self, -eps) + (eps - self.mu) * normal_exp(self, eps))
            / (alpha * (2. * PI).sqrt())
            - mu.powi(2);


        Gaussian { mu, sigma } / self
    }


    pub fn greater_eps(&self, eps: f64) -> Gaussian {
        let alpha = 0.5 - 0.5 * erf((eps - self.mu) / ((2. as f64).sqrt() * self.sigma));
        let mu = self.mu + self.sigma
            * normal_exp(self, -eps)
            / (alpha * (2. * PI).sqrt());
        let sigma = self.mu.powi(2) + self.sigma.powi(2) - self.sigma
            * ((eps + self.mu) * normal_exp(self, -eps))
            / (alpha * (2. * PI).sqrt())
            - mu.powi(2);


        Gaussian { mu, sigma } / self
    }
}
