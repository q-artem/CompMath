pub mod functions;
pub mod input;
pub mod methods;

use crate::data_io::{print_header, print_sep_line};
use crate::lab3::input::{
    choose_function, choose_lab_mode, choose_method, read_limits, read_precision,
};
use crate::lab3::methods::integrate_with_precision;

pub fn solve() {
    print_header("Лабораторная работа №3. Численное интегрирование", 1);

    loop {
        let mode = match choose_lab_mode() {
            Some(0) => break,
            Some(m) => m,
            None => break,
        };

        let is_improper = mode == 2;

        let function_data = match choose_function(is_improper) {
            Some(f) => f,
            None => continue,
        };

        let method = choose_method();
        let (mut a, mut b) = read_limits();
        let epsilon = read_precision();
        let initial_n = 4;

        let mut multiplier = 1.0;
        if a > b {
            std::mem::swap(&mut a, &mut b);
            multiplier = -1.0;
        }

        print_sep_line(2);
        println!(
            "Вычисляем {} интеграл f(x) = {} на интервале [{:.3}, {:.3}]",
            if is_improper {
                "несобственный"
            } else {
                "собственный"
            },
            function_data.description,
            a,
            b
        );
        println!(
            "Метод: {}",
            if is_improper {
                "Метод средних прямоугольников (адаптивный)"
            } else {
                method.name()
            }
        );
        println!("Требуемая точность: {}", epsilon);

        match integrate_with_precision(
            method,
            &function_data,
            a,
            b,
            epsilon,
            initial_n,
            is_improper,
        ) {
            Ok(result) => {
                let final_value = result.value * multiplier;
                print_header("РЕЗУЛЬТАТ:", 3);
                println!("Значение интеграла: {:.10}", final_value);
                println!("Число разбиений (n): {}", result.n);
                println!("Оценочная погрешность (по Рунге): {:.10}", result.error);
            }
            Err(e) => {
                println!("\nРЕЗУЛЬТАТ:");
                println!("{}", e);
            }
        }
    }
}
