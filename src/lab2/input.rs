use crate::{
    data_io::{print_header, read_choice, read_f64},
    lab2::equations::{EquationFn, get_non_linear_functions},
};
use std::io::{self, Write};

pub struct EquationTask {
    pub f: EquationFn,
    pub a: f64,
    pub b: f64,
    pub epsilon: f64,
}

pub fn select_non_linear() -> EquationTask {
    let f = select_equation();
    print_header("Ввод границ и точности", 3);
    let a = read_f64("Введите начало интервала (a): ");
    let b = read_f64("Введите конец интервала (b): ");
    let epsilon = read_f64("Введите точность (epsilon): ");
    return EquationTask { f, a, b, epsilon };
}

pub fn select_equation() -> EquationFn {
    let functions = get_non_linear_functions();
    let mut names: Vec<&&str> = functions.keys().collect();
    names.sort();

    loop {
        print_header("Выбор уравнения", 3);
        for (i, name) in names.iter().enumerate() {
            println!("{}. {}", i + 1, name);
        }
        print!("Выберите номер: ");
        io::stdout().flush().unwrap();

        if let Some(n) = read_choice() {
            if n > 0 && n <= (names.len() as u32) {
                let name = names[(n - 1) as usize];
                return *functions.get(name).unwrap();
            }
        }
        println!("Ошибка. Попробуйте еще раз");
    }
}
