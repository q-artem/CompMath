pub type MathFunction = fn(f64) -> f64;

#[derive(Clone, Copy)]
pub struct Singularity {
    pub point: f64,
    pub p_order: f64, // Порядок особенности: 1/|x-c|^p. Сходится если p < 1
}

pub struct IntegralFunction {
    pub description: &'static str,
    pub func: MathFunction,
    pub singularities: Vec<Singularity>,
}

pub fn get_functions() -> Vec<IntegralFunction> {
    vec![
        IntegralFunction {
            description: "x^3 - 3x^2 + 5x + 12",
            func: |x| x.powi(3) - 3.0 * x.powi(2) + 5.0 * x + 12.0,
            singularities: vec![],
        },
        IntegralFunction {
            description: "sin(x)",
            func: |x| x.sin(),
            singularities: vec![],
        },
        IntegralFunction {
            description: "e^x",
            func: |x| x.exp(),
            singularities: vec![],
        },
        IntegralFunction {
            description: "1/x",
            func: |x| 1.0 / x,
            singularities: vec![Singularity {
                point: 0.0,
                p_order: 1.0,
            }],
        },
        IntegralFunction {
            description: "x^2",
            func: |x| x.powi(2),
            singularities: vec![],
        },
    ]
}

pub fn get_improper_functions() -> Vec<IntegralFunction> {
    vec![
        IntegralFunction {
            description: "1/sqrt(x)",
            func: |x| 1.0 / x.sqrt(),
            singularities: vec![Singularity {
                point: 0.0,
                p_order: 0.5,
            }],
        },
        IntegralFunction {
            description: "1/(x-2)^2",
            func: |x| 1.0 / (x - 2.0).powi(2),
            singularities: vec![Singularity {
                point: 2.0,
                p_order: 2.0,
            }],
        },
        IntegralFunction {
            description: "1/cbrt(x-1)",
            func: |x| 1.0 / (x - 1.0).cbrt(),
            singularities: vec![Singularity {
                point: 1.0,
                p_order: 0.333333333,
            }],
        },
    ]
}
