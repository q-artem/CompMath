use crate::{
    data_io::{print_header, print_sep_line, read_choice},
    lab2::{equations::EquationFn, input::EquationTask},
    lab2::utils::plot_equation,
};

use std::io::{self, Write};

pub enum RootCount {
    Zero,
    One,
    Multiple(usize),
}

pub struct MethodResult {
    pub root: f64,
    pub iterations: usize,
    pub f_value: f64,
}

pub fn analyze_roots(f: EquationFn, a: f64, b: f64) -> RootCount {
    let steps = 10000;
    let h = (b - a) / steps as f64;
    let mut count = 0;

    let mut prev_val = f(a);

    for i in 1..=steps {
        let x = a + (i as f64) * h;
        let current_val = f(x);

        if prev_val.is_finite() && current_val.is_finite() {
            if prev_val.signum() != current_val.signum() || current_val == 0.0 {
                count += 1;
            }
        }
        prev_val = current_val;
    }

    match count {
        0 => RootCount::Zero,
        1 => RootCount::One,
        n => RootCount::Multiple(n),
    }
}

pub fn analyse_and_solve_linear(task: &EquationTask) {
    if task.a >= task.b {
        println!("ОШИБКА: Левая граница (a) должна быть меньше правой (b)!");
        return;
    }
    
    match plot_equation(task.f, task.a, task.b, "graph.png") {
        Ok(_) => println!("График успешно сохранен в файл graph.png"),
        Err(e) => println!("Ошибка при рисовании графика: {}", e),
    }

    match analyze_roots(task.f, task.a, task.b) {
        RootCount::Zero => {
            println!("СООБЩЕНИЕ: На данном интервале корней не обнаружено.");
            println!("Попробуйте выбрать другой интервал.");
        }
        RootCount::Multiple(n) => {
            println!(
                "СООБЩЕНИЕ: На данном интервале обнаружено несколько корней (минимум {}).",
                n
            );
            println!(
                "Для корректной работы методов уточнения корня выберите более узкий интервал."
            );
        }
        RootCount::One => {
            println!("Верификация успешна: на интервале обнаружен один корень.");

            loop {
                println!("Выберите метод решения:");
                println!("1. Метод дихотомии");
                println!("2. Метод секущих");
                println!("3. Метод простых итераций");
                println!("0. Выход");

                print!("Выберите пункт: ");

                std::io::stdout().flush().unwrap();
                match read_choice() {
                    Some(1) => {
                        match dichotomy(task.f, task.a, task.b, task.epsilon) {
                            Ok(res) => print_result("Метод дихотомии", res),
                            Err(e) => println!("Ошибка в расчетах: {}", e), // Просто печатаем текст ошибки
                        }
                    }
                    Some(2) => match secant(task.f, task.a, task.b, task.epsilon) {
                        Ok(res) => print_result("Метод секущих", res),
                        Err(e) => println!("Ошибка в расчетах: {}", e),
                    },
                    Some(3) => match simple_iteration(task.f, task.a, task.b, task.epsilon) {
                        Ok(res) => print_result("Метод простых итераций", res),
                        Err(e) => println!("Ошибка в расчетах: {}", e),
                    },
                    Some(0) => return,
                    _ => println!("Ошибка. Попробуйте еще раз"),
                }
            }
        }
    }
}

pub fn print_result(method_name: &str, result: MethodResult) {
    print_header(method_name, 2);

    println!("{:>25}: {:.10}", "Найденный корень (x)", result.root);
    println!("{:>25}: {:.2e}", "Значение функции f(x)", result.f_value);
    println!("{:>25}: {}", "Количество итераций", result.iterations);

    print_sep_line(2);
}

pub fn dichotomy(f: EquationFn, mut a: f64, mut b: f64, eps: f64) -> Result<MethodResult, String> {
    if f(a) * f(b) > 0.0 {
        return Err("На концах отрезка функция должна иметь разные знаки!".to_string());
    }

    let mut iterations = 0;
    while (a - b).abs() > eps {
        iterations += 1;
        let x = (a + b) / 2.0;

        if f(a) * f(x) <= 0.0 {
            b = x;
        } else {
            a = x;
        }

        if iterations > 1000 {
            break;
        }
    }

    let root = (a + b) / 2.0;
    Ok(MethodResult {
        root,
        iterations,
        f_value: f(root),
    })
}

pub fn secant(f: EquationFn, a: f64, b: f64, eps: f64) -> Result<MethodResult, String> {
    let mut x_prev = a;
    let mut x_curr = b;
    let mut iterations = 0;

    loop {
        iterations += 1;
        let f_curr = f(x_curr);
        let f_prev = f(x_prev);

        if (f_curr - f_prev).abs() < 1e-15 {
            return Err("Деление на ноль в методе секущих (значения функции совпали)".to_string());
        }

        let x_next = x_curr - f_curr * (x_curr - x_prev) / (f_curr - f_prev);

        if (x_next - x_curr).abs() < eps {
            return Ok(MethodResult {
                root: x_next,
                iterations,
                f_value: f(x_next),
            });
        }

        x_prev = x_curr;
        x_curr = x_next;

        if iterations > 1000 {
            return Err("Превышено макс. количество итераций в методе секущих".to_string());
        }
    }
}

pub fn simple_iteration(f: EquationFn, a: f64, b: f64, eps: f64) -> Result<MethodResult, String> {
    // примерное значение производной на концах и в середине, чтобы найти лямбду
    let h = 0.001;
    let df_a = (f(a + h) - f(a)) / h;
    let df_b = (f(b + h) - f(b)) / h;

    // максимум производной
    let max_df = df_a.abs().max(df_b.abs());
    let lambda = 1.0 / max_df;

    let lambda = if df_a > 0.0 { -lambda } else { lambda };

    let mut x_curr = a; // Начальное приближение
    let mut iterations = 0;

    loop {
        iterations += 1;
        // x_{k+1} = x_k + lambda * f(x_k)
        let x_next = x_curr + lambda * f(x_curr);

        if (x_next - x_curr).abs() < eps {
            return Ok(MethodResult {
                root: x_next,
                iterations,
                f_value: f(x_next),
            });
        }

        x_curr = x_next;

        if iterations > 1000 {
            return Err(
                "Метод простых итераций не сходится (проверьте условие сходимости)".to_string(),
            );
        }
    }
}
