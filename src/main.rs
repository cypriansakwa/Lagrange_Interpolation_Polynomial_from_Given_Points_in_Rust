use std::fmt;

// Structure to hold polynomial terms
#[derive(Debug, Clone)]
struct Polynomial {
    coeffs: Vec<f64>,
}

impl Polynomial {
    // Create a new polynomial initialized to zero
    fn new() -> Self {
        Self { coeffs: vec![0.0] }
    }

    // Create a polynomial with a single term (constant)
    fn from_constant(c: f64) -> Self {
        Self { coeffs: vec![c] }
    }

    // Add two polynomials
    fn add(&mut self, other: &Polynomial) {
        let max_len = self.coeffs.len().max(other.coeffs.len());
        self.coeffs.resize(max_len, 0.0);
        for (i, &coeff) in other.coeffs.iter().enumerate() {
            self.coeffs[i] += coeff;
        }
    }

    // Multiply a polynomial by another polynomial
    fn multiply(&self, other: &Polynomial) -> Polynomial {
        let mut result = vec![0.0; self.coeffs.len() + other.coeffs.len() - 1];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                result[i + j] += a * b;
            }
        }
        Polynomial { coeffs: result }
    }

    // Multiply polynomial by a scalar value
    fn scale(&self, scalar: f64) -> Polynomial {
        Polynomial {
            coeffs: self.coeffs.iter().map(|&c| c * scalar).collect(),
        }
    }

    // Create a polynomial of the form (x - a)
    fn from_monomial(a: f64) -> Self {
        Self {
            coeffs: vec![-a, 1.0],
        }
    }
}

// Implement display formatting for the Polynomial struct
impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for (i, &coeff) in self.coeffs.iter().enumerate().rev() {
            if coeff != 0.0 {
                let term = if i == 0 {
                    format!("{:.2}", coeff)
                } else if i == 1 {
                    format!("{:.2}x", coeff)
                } else {
                    format!("{:.2}x^{}", coeff, i)
                };
                if !result.is_empty() && coeff > 0.0 {
                    result.push_str(&format!(" + {}", term));
                } else {
                    result.push_str(&term);
                }
            }
        }
        write!(f, "{}", result)
    }
}

// Lagrange interpolation function
fn lagrange_interpolation(points: &[(f64, f64)]) -> Polynomial {
    let mut result = Polynomial::new();

    for (i, &(x_i, y_i)) in points.iter().enumerate() {
        let mut term = Polynomial::from_constant(1.0);
        let mut denominator = 1.0;

        for (j, &(x_j, _)) in points.iter().enumerate() {
            if i != j {
                let poly = Polynomial::from_monomial(x_j);
                term = term.multiply(&poly);
                denominator *= x_i - x_j;
            }
        }

        term = term.scale(y_i / denominator);
        result.add(&term);
    }

    result
}

fn main() {
    let points = [(0.0, 4.0), (-2.0, 1.0), (2.0, 3.0)];

    let interpolated_polynomial = lagrange_interpolation(&points);

    println!("The interpolated polynomial is: {}", interpolated_polynomial);
}
