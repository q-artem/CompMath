pub fn gauss_seidel(
    a: &Vec<Vec<f64>>,
    b: &Vec<f64>,
    epsilon: f64,
    max_iter: usize,
) -> (Vec<f64>, usize, Vec<f64>) {
    let n = a.len();
    let mut x = vec![0.0; n]; // Начальное приближение
    let mut x_prev = vec![0.0; n];
    let mut errors = vec![0.0; n];
    let mut iter = 0;

    loop {
        x_prev.copy_from_slice(&x);
        let mut max_diff = 0.0_f64;

        for i in 0..n {
            let mut sum1 = 0.0;
            for j in 0..i {
                sum1 += a[i][j] * x[j];
            }

            let mut sum2 = 0.0;
            for j in (i + 1)..n {
                sum2 += a[i][j] * x_prev[j]; // Используем значения с прошлого шага
            }

            x[i] = (b[i] - sum1 - sum2) / a[i][i];

            let diff = (x[i] - x_prev[i]).abs();
            errors[i] = diff;
            if diff > max_diff {
                max_diff = diff;
            }
        }

        iter += 1;
        if max_diff < epsilon || iter >= max_iter {
            break;
        }
    }

    (x, iter, errors)
}

pub fn make_diagonally_dominant(a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>) -> bool {
    let n = a.len();
    let mut used_rows = vec![false; n];
    let mut new_a = vec![vec![0.0; n]; n];
    let mut new_b = vec![0.0; n];

    for i in 0..n {
        let mut found = false;

        for r in 0..n {
            if used_rows[r] {
                continue;
            }

            let mut sum = 0.0;
            for j in 0..n {
                if i != j {
                    sum += a[r][j].abs();
                }
            }

            // подходит ли строка r на позицию i
            if a[r][i].abs() >= sum {
                new_a[i] = a[r].clone();
                new_b[i] = b[r];
                used_rows[r] = true;
                found = true;
                break;
            }
        }

        if !found {
            return false; // не нашли
        }
    }

    *a = new_a;
    *b = new_b;
    true
}

pub fn matrix_norm(a: &Vec<Vec<f64>>) -> f64 {
    let n = a.len();
    let mut max_norm = 0.0_f64;

    for i in 0..n {
        let mut row_sum = 0.0;
        for j in 0..n {
            if i != j {
                row_sum += (a[i][j] / a[i][i]).abs();
            }
        }
        if row_sum > max_norm {
            max_norm = row_sum;
        }
    }
    max_norm
}

pub fn calculate_residuals(a: &Vec<Vec<f64>>, x: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let n = a.len();
    let mut r = vec![0.0; n];

    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..n {
            sum += a[i][j] * x[j];
        }
        r[i] = sum - b[i];
    }
    r
}
