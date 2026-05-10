use std::fmt;

#[derive(Debug, Clone)]
pub enum ModelType {
    Linear,
    Polynomial2,
    Polynomial3,
    Exponential,
    Logarithmic,
    Power,
}

impl fmt::Display for ModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelType::Linear => write!(f, "Линейная (y = ax + b)"),
            ModelType::Polynomial2 => write!(f, "Полиномиальная 2-й степени (y = ax^2 + bx + c)"),
            ModelType::Polynomial3 => {
                write!(f, "Полиномиальная 3-й степени (y = ax^3 + bx^2 + cx + d)")
            }
            ModelType::Exponential => write!(f, "Экспоненциальная (y = a*e^(bx))"),
            ModelType::Logarithmic => write!(f, "Логарифмическая (y = a*ln(x) + b)"),
            ModelType::Power => write!(f, "Степенная (y = a*x^b)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApproximationResult {
    pub model_type: ModelType,
    pub coefficients: Vec<f64>, // a, b, c, d
    pub s: f64,                 // Measure of deviation
    pub epsilon: f64,           // RMSD (Standard Deviation)
    pub r_pearson: Option<f64>, // Pearson correlation (for linear)
    pub r_squared: f64,         // Coefficient of determination
}

impl ApproximationResult {
    pub fn predict(&self, x: f64) -> f64 {
        let coeffs = &self.coefficients;
        match self.model_type {
            ModelType::Linear => coeffs[0] * x + coeffs[1],
            ModelType::Polynomial2 => coeffs[0] * x.powi(2) + coeffs[1] * x + coeffs[2],
            ModelType::Polynomial3 => {
                coeffs[0] * x.powi(3) + coeffs[1] * x.powi(2) + coeffs[2] * x + coeffs[3]
            }
            ModelType::Exponential => coeffs[0] * (coeffs[1] * x).exp(),
            ModelType::Logarithmic => coeffs[0] * x.ln() + coeffs[1],
            ModelType::Power => coeffs[0] * x.powf(coeffs[1]),
        }
    }

    pub fn formula(&self) -> String {
        let c = &self.coefficients;
        match self.model_type {
            ModelType::Linear => format!("y = {:.4}x + {:.4}", c[0], c[1]),
            ModelType::Polynomial2 => format!("y = {:.4}x^2 + {:.4}x + {:.4}", c[0], c[1], c[2]),
            ModelType::Polynomial3 => format!(
                "y = {:.4}x^3 + {:.4}x^2 + {:.4}x + {:.4}",
                c[0], c[1], c[2], c[3]
            ),
            ModelType::Exponential => format!("y = {:.4} * e^({:.4}x)", c[0], c[1]),
            ModelType::Logarithmic => format!("y = {:.4} * ln(x) + {:.4}", c[0], c[1]),
            ModelType::Power => format!("y = {:.4} * x^{:.4}", c[0], c[1]),
        }
    }
}
