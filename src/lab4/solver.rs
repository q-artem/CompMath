use crate::lab4::functions::Point;
use crate::lab4::models::{ApproximationResult, ModelType};

pub fn solve_lsm(points: &[Point]) -> Vec<ApproximationResult> {
    let mut results = Vec::new();

    if let Some(res) = solve_linear(points) {
        results.push(res);
    }
    if let Some(res) = solve_polynomial(points, 2) {
        results.push(res);
    }
    if let Some(res) = solve_polynomial(points, 3) {
        results.push(res);
    }
    if let Some(res) = solve_exponential(points) {
        results.push(res);
    }
    if let Some(res) = solve_logarithmic(points) {
        results.push(res);
    }
    if let Some(res) = solve_power(points) {
        results.push(res);
    }

    results
}

fn solve_linear(points: &[Point]) -> Option<ApproximationResult> {
    let n = points.len() as f64;
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sxx = 0.0;
    let mut sxy = 0.0;

    for p in points {
        sx += p.x;
        sy += p.y;
        sxx += p.x * p.x;
        sxy += p.x * p.y;
    }

    let det = sxx * n - sx * sx;
    if det.abs() < 1e-9 {
        return None;
    }

    let a = (sxy * n - sx * sy) / det;
    let b = (sxx * sy - sx * sxy) / det;

    // Pearson correlation
    let x_mean = sx / n;
    let y_mean = sy / n;
    let mut num = 0.0;
    let mut den_x = 0.0;
    let mut den_y = 0.0;
    for p in points {
        num += (p.x - x_mean) * (p.y - y_mean);
        den_x += (p.x - x_mean).powi(2);
        den_y += (p.y - y_mean).powi(2);
    }
    let r_pearson = if den_x * den_y > 0.0 {
        Some(num / (den_x * den_y).sqrt())
    } else {
        None
    };

    Some(build_result(
        ModelType::Linear,
        vec![a, b],
        points,
        r_pearson,
    ))
}

fn solve_polynomial(points: &[Point], degree: usize) -> Option<ApproximationResult> {
    let n = degree + 1;
    let mut matrix = vec![vec![0.0; n]; n];
    let mut b = vec![0.0; n];

    for i in 0..n {
        for j in 0..n {
            let p_sum = points
                .iter()
                .map(|p| p.x.powi((degree * 2 - (i + j)) as i32))
                .sum::<f64>();
            matrix[i][j] = p_sum;
        }
        b[i] = points
            .iter()
            .map(|p| p.y * p.x.powi((degree - i) as i32))
            .sum::<f64>();
    }

    if let Some(coeffs) = solve_gaussian(matrix, b) {
        let model_type = if degree == 2 {
            ModelType::Polynomial2
        } else {
            ModelType::Polynomial3
        };
        Some(build_result(model_type, coeffs, points, None))
    } else {
        None
    }
}

fn solve_exponential(points: &[Point]) -> Option<ApproximationResult> {
    if points.iter().any(|p| p.y <= 0.0) {
        return None;
    }

    let transformed_points: Vec<Point> = points
        .iter()
        .map(|p| Point {
            x: p.x,
            y: p.y.ln(),
        })
        .collect();
    if let Some(lin) = solve_linear(&transformed_points) {
        let b = lin.coefficients[0];
        let a = lin.coefficients[1].exp();
        Some(build_result(
            ModelType::Exponential,
            vec![a, b],
            points,
            None,
        ))
    } else {
        None
    }
}

fn solve_logarithmic(points: &[Point]) -> Option<ApproximationResult> {
    if points.iter().any(|p| p.x <= 0.0) {
        return None;
    }

    let transformed_points: Vec<Point> = points
        .iter()
        .map(|p| Point {
            x: p.x.ln(),
            y: p.y,
        })
        .collect();
    if let Some(lin) = solve_linear(&transformed_points) {
        let a = lin.coefficients[0];
        let b = lin.coefficients[1];
        Some(build_result(
            ModelType::Logarithmic,
            vec![a, b],
            points,
            None,
        ))
    } else {
        None
    }
}

fn solve_power(points: &[Point]) -> Option<ApproximationResult> {
    if points.iter().any(|p| p.x <= 0.0 || p.y <= 0.0) {
        return None;
    }

    let transformed_points: Vec<Point> = points
        .iter()
        .map(|p| Point {
            x: p.x.ln(),
            y: p.y.ln(),
        })
        .collect();
    if let Some(lin) = solve_linear(&transformed_points) {
        let b = lin.coefficients[0];
        let a = lin.coefficients[1].exp();
        Some(build_result(ModelType::Power, vec![a, b], points, None))
    } else {
        None
    }
}

fn build_result(
    model_type: ModelType,
    coefficients: Vec<f64>,
    points: &[Point],
    r_pearson: Option<f64>,
) -> ApproximationResult {
    let mut res = ApproximationResult {
        model_type,
        coefficients,
        s: 0.0,
        epsilon: 0.0,
        r_pearson,
        r_squared: 0.0,
    };

    let mut s = 0.0;
    let mut sy = 0.0;
    let mut syy = 0.0;
    for p in points {
        let y_pred = res.predict(p.x);
        s += (y_pred - p.y).powi(2);
        sy += p.y;
        syy += p.y.powi(2);
    }
    res.s = s;
    res.epsilon = (s / points.len() as f64).sqrt();

    // R^2 = 1 - S / SS_tot
    let n = points.len() as f64;
    let ss_tot = syy - (sy * sy) / n;
    res.r_squared = if ss_tot > 0.0 { 1.0 - s / ss_tot } else { 1.0 };

    res
}

fn solve_gaussian(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for i in 0..n {
        let mut max_row = i;
        for k in i + 1..n {
            if a[k][i].abs() > a[max_row][i].abs() {
                max_row = k;
            }
        }
        a.swap(i, max_row);
        b.swap(i, max_row);

        if a[i][i].abs() < 1e-12 {
            return None;
        }

        for k in i + 1..n {
            let c = -a[k][i] / a[i][i];
            for j in i..n {
                if i == j {
                    a[k][j] = 0.0;
                } else {
                    a[k][j] += c * a[i][j];
                }
            }
            b[k] += c * b[i];
        }
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        x[i] = b[i] / a[i][i];
        for k in 0..i {
            b[k] -= a[k][i] * x[i];
        }
    }
    Some(x)
}
