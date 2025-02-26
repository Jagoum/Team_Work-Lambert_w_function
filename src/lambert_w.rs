 use thiserror::Error;
 use std::f64::consts::E;

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
