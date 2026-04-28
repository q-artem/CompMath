use crate::data_io::{print_header, read_choice, read_f64};
use crate::lab3::functions::{IntegralFunction, get_functions, get_improper_functions};
use crate::lab3::methods::IntegrationMethodType;

pub fn choose_lab_mode() -> Option<u32> {
    print_header("Выберите режим работы", 2);
    println!("1. Вычисление собственных интегралов");
    println!("2. Работа с несобственными интегралами (Доп)");
    println!("0. Выход");

    loop {
        print!("Выберите пункт: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        if let Some(choice) = read_choice() {
            if choice <= 2 {
                return Some(choice);
            }
        }
        println!("Ошибка: выберите 0, 1 или 2.");
    }
}

pub fn choose_function(is_improper: bool) -> Option<IntegralFunction> {
    let functions = if is_improper {
        get_improper_functions()
    } else {
        get_functions()
    };
    let title = if is_improper {
        "Выберите функцию с особенностями"
    } else {
        "Выберите функцию для интегрирования"
    };

    print_header(title, 3);
    for (i, f) in functions.iter().enumerate() {
        println!("{}. f(x) = {}", i + 1, f.description);
    }
    println!("0. Назад");

    loop {
        print!("Выберите номер (0-{}): ", functions.len());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        if let Some(choice) = read_choice() {
            if choice == 0 {
                return None;
            }
            if choice >= 1 && choice <= functions.len() as u32 {
                return Some(if is_improper {
                    get_improper_functions().remove((choice - 1) as usize)
                } else {
                    get_functions().remove((choice - 1) as usize)
                });
            }
        }
        println!("Ошибка: введите корректный номер.");
    }
}

pub fn choose_method() -> IntegrationMethodType {
    print_header("Выберите метод интегрирования", 3);
    println!("1. Метод левых прямоугольников");
    println!("2. Метод правых прямоугольников");
    println!("3. Метод средних прямоугольников");
    println!("4. Метод трапеций");
    println!("5. Метод Симпсона");

    loop {
        print!("Выберите номер (1-5): ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        if let Some(choice) = read_choice() {
            match choice {
                1 => return IntegrationMethodType::LeftRectangle,
                2 => return IntegrationMethodType::RightRectangle,
                3 => return IntegrationMethodType::MidpointRectangle,
                4 => return IntegrationMethodType::Trapezoid,
                5 => return IntegrationMethodType::Simpson,
                _ => {}
            }
        }
        println!("Ошибка: введите число от 1 до 5.");
    }
}

pub fn read_limits() -> (f64, f64) {
    let a = read_f64("Введите нижний предел интегрирования (a): ");
    let b = read_f64("Введите верхний предел интегрирования (b): ");
    if a > b {
        println!("Предупреждение: a > b. Пределы будут поменяны местами, результат умножен на -1.");
    }
    (a, b)
}

pub fn read_precision() -> f64 {
    loop {
        print!("Введите точность (по умолчанию 0.0001): ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return 0.0001;
        }

        if let Ok(eps) = trimmed.replace(',', ".").parse::<f64>() {
            if eps > 0.0 {
                return eps;
            }
        }
        println!("Ошибка: точность должна быть положительным числом.");
    }
}
