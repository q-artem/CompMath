use io::Align::{Center, Right};
use io::{print, print_header, print_menu, read_choice};

use input::read_matrix_from_file;

use input::gen_random_matrix;

use crate::io::print_matrix;

mod input;
mod io;
mod math;

fn main() {
    print_header("Вычислительная математика. Лабораторная работа №1", 1);
    print("Пшеничников Артём Дмитриевич, P3207, 467205", Right);
    print_header("Вариант 6. Метод Гаусса-Зейделя", 2);

    loop {
        print_menu();
        match read_choice() {
            Some(1) => {
                print_header("ВВОД С КЛАВИАТУРЫ", 3);
                match input::read_matrix_from_keyboard() {
                    Ok((a, b, eps)) => solve_and_print(a, b, eps),
                    Err(e) => println!("ОШИБКА ВВОДА: {}", e),
                }
            }
            Some(2) => {
                print_header("ЧТЕНИЕ ИЗ ФАЙЛА", 3);
                match read_matrix_from_file() {
                    Ok((a, b, eps)) => solve_and_print(a, b, eps),
                    Err(e) => println!("ОШИБКА: {}", e),
                }
            }
            Some(3) => {
                print_header("ТЕСТОВЫЙ ПРИМЕР", 3);
                let a = vec![
                    vec![2.0, 2.0, 10.0],
                    vec![10.0, 1.0, 1.0],
                    vec![2.0, 10.0, 1.0],
                ];
                let b = vec![14.0, 12.0, 13.0];
                solve_and_print(a, b, 0.01);
            }
            Some(4) => {
                print_header("ГЕНЕРАЦИЯ МАТРИЦЫ", 3);
                match gen_random_matrix() {
                    Ok((a, b, eps)) => solve_and_print(a, b, eps),
                    Err(e) => println!("ОШИБКА: {}", e),
                }
            }
            Some(42) => {
                print("42", Center);
            }
            Some(0) => {
                print_header("ЗАВЕРШЕНИЕ РАБОТЫ. СПАСИБО ЗА ИСПОЛЬЗОВАНИЕ ПРОГРАММЫ", 1);
                break;
            }
            _ => println!("Ошибка. Попробуйте еще раз, число от 0 до 3"),
        }
    }
}

fn solve_and_print(mut a: Vec<Vec<f64>>, mut b: Vec<f64>, epsilon: f64) {
    let a_original = a.clone();
    let b_original = b.clone();

    if !math::make_diagonally_dominant(&mut a, &mut b) {
        println!("Диагональное преобладание не достигнуто. Метод может не сойтись.");
    } else {
        println!(
            "Матрица успешно преобразована для диагонального преобладания. Преобразованная матрица:"
        );
        print_matrix(&a, &b, 2);
    }

    let norm = math::matrix_norm(&a);
    println!("Норма матрицы преобразования C: {:.4}", norm);
    if norm >= 1.0 {
        println!("Предупреждение: Норма >= 1, достаточный признак сходимости не выполнен.");
    }
    let (x, iters, errors) = math::gauss_seidel(&a, &b, epsilon, 1000);
    let residuals = math::calculate_residuals(&a_original, &x, &b_original);

    print_header("РЕЗУЛЬТАТЫ ВЫЧИСЛЕНИЙ", 2);
    println!("Количество итераций: {}", iters);
    println!();

    println!(
        "{:>3} | {:>12} | {:>12} | {:>12}",
        "№", "x[i]", "Невязка", "Погрешность"
    );
    println!("{}", "-".repeat(50));

    for i in 0..x.len() {
        println!(
            "{:>3} | {:>12.6} | {:>12.2e} | {:>12.2e}",
            i + 1,
            x[i],
            residuals[i],
            errors[i]
        );
    }
    println!("{}", "-".repeat(50));
}
