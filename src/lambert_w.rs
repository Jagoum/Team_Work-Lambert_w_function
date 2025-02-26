use std::f64::consts::E;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("value should be more than -1/e")]
    Invalidinput,
}
pub fn lambert_function(x: f64) -> Result<f64, Error> {
    let div = -1.0 / E;

    if x < div {
        return Err(Error::Invalidinput);
    }
    let mut w_next = 0.0;

    // Initial guess
    let mut w = if x == 0.0 {
        0.0
    } else if x < 1.0 {
        x * (1.0 - x.exp()) // Better initial guess for small positive numbers
    } else {
        x.ln() // Initial guess for x >= 1.0
    };

    for _ in 0..500 {
        let ew = w.exp();
        let wew = w * ew;
        let l = wew - x;
        let l_prime = ew * (w + 1.0);

        // Pure Newton-Raphson update
        w_next = w - l / l_prime;

        // convergence tolerance
        if (w_next - w).abs() < 1e-7 {
            return Ok(w_next);
        }

        w = w_next;
    }
    Ok(w_next)
}

//use crate::src::lambert_w::lambert_function;

#[cfg(test)]
mod lambert {
    use super::*;
    use log::info;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn lambert_w_pos_input() {
        init();

        info!("lambert for values positive values");
        let result = lambert_function(1.0);
        assert_eq!(result.unwrap(), 0.567143290409784);
    }

    #[test]
    fn lambert_w_principal_branch() {
        init();

        info!("The principle branch w(0)");
        assert_eq!(lambert_function(0.0).unwrap(), 0.0);
    }
    #[test]
    fn lambert_function_neg_val() {
        init();

        info!("lambert a value less than -1/e");
        assert_eq!(lambert_function(-0.2).unwrap(), -0.2591711018190738);
    }

    #[test]
    fn lambert_w_larger_val() {
        init();

        info!("lambert for large values");
        assert_eq!(
            lambert_function(10000000000000000000000000000.0).unwrap(),
            60.371859509617295
        );
    }
}
