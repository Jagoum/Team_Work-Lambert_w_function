use std::f64::consts::E;

use anyhow::Result;

pub fn lambert_w(x: f64) -> Result<f64> {
    // x must be > -1/e
    if x < -1.0 / std::f64::consts::E {
        return Err(anyhow::anyhow!("Input out of domain: x must be >= -1/e"));
    }
 

     // Initial guess
     let mut w = if x == 0.0 {
        0.0
    } else if x < 1.0 {
        x * (1.0 - x.exp()) // Better initial guess for small positive numbers
    } else {
        x.ln() // Initial guess for x >= 1.0
    };

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
