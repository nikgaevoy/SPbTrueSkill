use super::*;

extern crate rand;

use rand::prelude::*;

#[test]
fn mean() {
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let a = Gaussian { mu: rng.gen(), sigma: rng.gen() };
        let b = Gaussian { mu: rng.gen(), sigma: a.sigma };
        let c = (&a + &b) * 0.5;

        assert!(approx_eq!(f64, c.mu, (a.mu + b.mu) / 2.));
    }
}

#[test]
fn multiplication() {
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let a = Gaussian { mu: rng.gen(), sigma: rng.gen() };
        let b = Gaussian { mu: rng.gen(), sigma: a.sigma };
        let c = &a * &b;

        assert!(approx_eq!(f64, c.mu, (a.mu + b.mu) / 2.));
    }
}

#[test]
fn multiplication_on_1() {
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let a = Gaussian { mu: rng.gen(), sigma: rng.gen() };
        let b = ONE;
        let c = &a * &b;

        assert!(approx_eq!(f64, c.mu, a.mu));
        assert!(approx_eq!(f64, c.sigma, a.sigma));
    }
}

#[test]
fn division_on_1() {
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let a = Gaussian { mu: rng.gen(), sigma: rng.gen() };
        let b = ONE;
        let c = &a / &b;

        assert!(approx_eq!(f64, c.mu, a.mu));
        assert!(approx_eq!(f64, c.sigma, a.sigma));
    }
}

#[test]
fn muldiv() {
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let a = Gaussian { mu: rng.gen_range(100., 200.), sigma: rng.gen() };
        let b = Gaussian { mu: rng.gen(), sigma: rng.gen_range(300., 700.) };
        let d = &a * &b;
        let c = &d / &b;

        assert!(approx_eq!(f64, c.mu, a.mu));
        assert!(approx_eq!(f64, c.sigma, a.sigma));
    }
}

#[test]
fn divmul() {
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        let a = Gaussian { mu: rng.gen_range(100., 200.), sigma: rng.gen() };
        let b = Gaussian { mu: rng.gen(), sigma: rng.gen_range(300., 700.) };
        let d = &a / &b;
        let c = &d * &b;

        assert!(approx_eq!(f64, c.mu, a.mu));
        assert!(approx_eq!(f64, c.sigma, a.sigma));
    }
}

#[test]
#[should_panic]
fn muldiv_numerical_stability() {
    let mut rng = rand::thread_rng();

    for _i in 0..100 {
        let a = Gaussian { mu: rng.gen(), sigma: rng.gen() };
        let b = Gaussian { mu: rng.gen(), sigma: rng.gen() };
        let d = &a * &b;
        let c = &d / &b;

        assert!(approx_eq!(f64, c.mu, a.mu));
        assert!(approx_eq!(f64, c.sigma, a.sigma));
    }
}

#[test]
#[should_panic]
fn divmul_numerical_stability() {
    let mut rng = rand::thread_rng();

    for _i in 0..100 {
        let a = Gaussian { mu: rng.gen(), sigma: rng.gen() };
        let b = Gaussian { mu: rng.gen(), sigma: rng.gen() };
        let d = &a / &b;
        let c = &d * &b;

        assert!(approx_eq!(f64, c.mu, a.mu));
        assert!(approx_eq!(f64, c.sigma, a.sigma));
    }
}

#[test]
#[should_panic]
fn div_0() {
    let mut rng = rand::thread_rng();

    let a = Gaussian { mu: rng.gen(), sigma: rng.gen() };
    let c = &a / &a;

    assert!(c.mu.is_finite());
    assert!(c.sigma.is_finite());
}