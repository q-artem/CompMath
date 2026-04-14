use std::io::{self, Write};
pub mod input;

use crate::{
    data_io::{print_header, print_sep_line, read_choice},
};

use crate::lab2::solvers::analyse_and_solve_linear;

pub mod equations;
pub mod solvers;
pub mod utils;

pub(crate) fn solve() {
    loop {
        print_menu();
        match read_choice() {
            Some(1) => {
                let f = input::select_non_linear();
                analyse_and_solve_linear(&f);
            }
            Some(2) => {}
            Some(0) => break,
            _ => println!("Ошибка. Попробуйте еще раз"),
        }
    }
}

fn print_menu() {
    print_header("ГЛАВНОЕ МЕНЮ", 2);
    println!("1.  Нелинейное уравнение");
    println!("2.  Система нелинейных уравнений");
    println!("0.  Выход");
    print_sep_line(2);
    print!("Выберите пункт: ");

    io::stdout().flush().unwrap();
}
