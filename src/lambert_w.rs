 use thiserror::Error;
 use std::f64::consts::E;

#[derive(Debug, Error)]
pub enum Error {
    #[error("value should be more than -1/e")]
    Invalidinput,

    #[error("value should be less than 0")]
    ZeroValue,
}


pub fn lambert_function(x: f64) -> Result<f64, Error> {
    let div = -1.0 / E;

    if x < div {
        return Err(Error::Invalidinput);
    }

    if x > 0.0 {
        return Err(Error::ZeroValue);
    }

    let w = x;
    println!("{}", x);

    let iter = 50;
    let tolerance = 1e-10;
    let mut wl = 0.0;
    for _ in 0..iter {
        let f = w * E.powf(w) - x;

        let f_prime = E.powf(w) * w + E.powf(w);

        wl = w - f / f_prime;

        if (wl - w).abs() < tolerance {
            wl = w
        }
        // println!("Iteration {}: w = {}, f(w) = {}, f'(w) = {}", i, w, f, f_prime);
    }
    Ok(wl)
}
