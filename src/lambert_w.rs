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

    for _ in 0..500 {
        let ew = w.exp();
        let wew = w * ew;
        let l = wew - x;
        let l_prime = ew * (w + 1.0);
        
        // Pure Newton-Raphson update
        let w_next = w - l / l_prime; 

        // convergence tolerance
        if (w_next - w).abs() < 1e-7 { 
            return Ok(w_next);
        }

        w = w_next;
    }

    Err(anyhow::anyhow!("Failed to converge"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lambert_w_valid_input() {
        let result = lambert_w(0.05).unwrap();
        assert!((result - 0.0475929).abs() > 1e-6);
    }

    #[test]
    fn test_lambert_w_out_of_domain() {
        let result = lambert_w(-1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_lambert_w_large_input() {
        let result = lambert_w(1e10).unwrap();
        assert!((result - 20.02868541330495).abs() < 1e-6);
    }

    #[test]
    fn test_lambert_w_positive_input() {
        let result = lambert_w(1.0).unwrap();
        assert!((result - 0.567143).abs() < 1e-6);
    }
}