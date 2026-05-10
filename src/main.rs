use data_io::print_header;
use std::io::Write;

use crate::data_io::Align::Right;
use crate::data_io::{print, print_sep_line, read_float_choice};

mod data_io;
mod lab1;
mod lab2;
mod lab3;
mod lab4;

fn main() {
    print_header("Вычислительная математика. Лабораторные работы", 1);
    print("Пшеничников Артём Дмитриевич, P3207, 467205", Right);

    loop {
        print_header("Выберите лабораторную для запуска", 2);
        println!("1.   Лабораторная работа 1");
        println!("2.   Лабораторная работа 2 (CLI)");
        println!("2.1. Лабораторная работа 2 (UI)");
        println!("3.   Лабораторная работа 3 (Численное интегрирование)");
        println!("4.   Лабораторная работа 4 (UI)");
        println!("0.   Выход");

        print_sep_line(2);
        print!("Выберите пункт: ");
        std::io::stdout().flush().unwrap();

        match read_float_choice() {
            Some(1.0) => {
                lab1::solve();
            }
            Some(2.0) => {
                lab2::solve();
            }
            Some(2.1) => {
                lab2::ui::run_ui();
            }
            Some(3.0) => {
                lab3::solve();
            }
            Some(4.0) => {
                lab4::ui::run_ui();
            }
            Some(0.0) => break,
            _ => println!("Ошибка. Введите корректный номер лабораторной"),
        }
    }
    print_header("ЗАВЕРШЕНИЕ РАБОТЫ. СПАСИБО ЗА ИСПОЛЬЗОВАНИЕ ПРОГРАММЫ", 1);
}
