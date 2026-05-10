use crate::lab3::functions::{IntegralFunction, MathFunction};

#[derive(Clone, Copy, PartialEq)]
pub enum IntegrationMethodType {
    LeftRectangle,
    RightRectangle,
    MidpointRectangle,
    Trapezoid,
    Simpson,
}

impl IntegrationMethodType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LeftRectangle => "Метод левых прямоугольников",
            Self::RightRectangle => "Метод правых прямоугольников",
            Self::MidpointRectangle => "Метод средних прямоугольников",
            Self::Trapezoid => "Метод трапеций",
            Self::Simpson => "Метод Симпсона",
        }
    }

    pub fn algebraic_order(&self) -> u32 {
        match self {
            Self::LeftRectangle | Self::RightRectangle => 1,
            Self::MidpointRectangle | Self::Trapezoid => 2,
            Self::Simpson => 4,
        }
    }
}

pub fn check_convergence(f_data: &IntegralFunction, a: f64, b: f64) -> Result<(), String> {
    for sing in &f_data.singularities {
        if sing.point >= a && sing.point <= b {
            if sing.p_order >= 1.0 {
                return Err(format!(
                    "Интеграл не существует (расходится). Обнаружена особенность в точке x = {} с порядком p = {:.2} >= 1.",
                    sing.point, sing.p_order
                ));
            }
        }
    }
    Ok(())
}

pub fn integrate(method: IntegrationMethodType, f: MathFunction, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;

    match method {
        IntegrationMethodType::LeftRectangle => {
            for i in 0..n {
                sum += f(a + i as f64 * h);
            }
            sum * h
        }
        IntegrationMethodType::RightRectangle => {
            for i in 1..=n {
                sum += f(a + i as f64 * h);
            }
            sum * h
        }
        IntegrationMethodType::MidpointRectangle => {
            for i in 0..n {
                sum += f(a + (i as f64 + 0.5) * h);
            }
            sum * h
        }
        IntegrationMethodType::Trapezoid => {
            sum = (f(a) + f(b)) / 2.0;
            for i in 1..n {
                sum += f(a + i as f64 * h);
            }
            sum * h
        }
        IntegrationMethodType::Simpson => {
            sum = f(a) + f(b);
            for i in 1..n {
                let x = a + i as f64 * h;
                if i % 2 == 0 {
                    sum += 2.0 * f(x);
                } else {
                    sum += 4.0 * f(x);
                }
            }
            sum * h / 3.0
        }
    }
}

pub fn integrate_improper(
    _method: IntegrationMethodType,
    f_data: &IntegralFunction,
    a: f64,
    b: f64,
    n: usize,
) -> f64 {
    let mut points = vec![a, b];
    for sing in &f_data.singularities {
        if sing.point > a && sing.point < b {
            points.push(sing.point);
        }
    }
    points.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let mut total_integral = 0.0;
    for i in 0..points.len() - 1 {
        let sub_a = points[i];
        let sub_b = points[i + 1];
        total_integral += integrate(
            IntegrationMethodType::MidpointRectangle,
            f_data.func,
            sub_a,
            sub_b,
            n,
        );
    }
    total_integral
}

pub struct IntegrationResult {
    pub value: f64,
    pub n: usize,
    pub error: f64,
}

pub fn integrate_with_precision(
    method: IntegrationMethodType,
    f_data: &IntegralFunction,
    a: f64,
    b: f64,
    epsilon: f64,
    mut n: usize,
    is_improper: bool,
) -> Result<IntegrationResult, String> {
    // 1. Проверка области определения
    let samples = 20;
    for i in 0..=samples {
        let x = a + (b - a) * (i as f64 / samples as f64);
        let val = (f_data.func)(x);
        // Для несобственных допускаем NaN в самих точках разрыва, но не на интервалах
        if val.is_nan() {
            let mut is_near_singularity = false;
            for sing in &f_data.singularities {
                if (x - sing.point).abs() < 1e-9 {
                    is_near_singularity = true;
                    break;
                }
            }
            if !is_near_singularity {
                return Err(
                    "Ошибка: функция не определена на данном интервале (получено NaN).".to_string(),
                );
            }
        }
    }

    if is_improper {
        check_convergence(f_data, a, b)?;
    }

    if matches!(method, IntegrationMethodType::Simpson) && n % 2 != 0 {
        n += 1;
    }

    let k = if is_improper {
        2
    } else {
        method.algebraic_order()
    };

    let mut i_n = if is_improper {
        integrate_improper(method, f_data, a, b, n)
    } else {
        integrate(method, f_data.func, a, b, n)
    };

    loop {
        if n > 1_048_576 {
            return Err(
                "Превышено максимальное число разбиений. Интеграл может расходиться.".to_string(),
            );
        }

        let n_new = n * 2;
        let i_2n = if is_improper {
            integrate_improper(method, f_data, a, b, n_new)
        } else {
            integrate(method, f_data.func, a, b, n_new)
        };

        if i_2n.is_nan() || i_2n.is_infinite() {
            return Err(
                "В процессе вычисления получено неопределенное значение (NaN/Inf).".to_string(),
            );
        }

        let runge_error = (i_2n - i_n).abs() / ((2.0_f64.powi(k as i32)) - 1.0);

        if runge_error <= epsilon {
            let mut val = i_2n;
            if val.abs() < epsilon * 0.01 {
                val = 0.0;
            } // Очистка околонулевых значений

            return Ok(IntegrationResult {
                value: val,
                n: n_new,
                error: runge_error,
            });
        }

        i_n = i_2n;
        n = n_new;
    }
}
