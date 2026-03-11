use crate::io::Align::Left;
use crate::io::{print, print_matrix};
use rand::Rng;
use std::fs;
use std::io::{self, Write};

pub fn read_matrix_from_file() -> Result<(Vec<Vec<f64>>, Vec<f64>, f64), String> {
    print!("Введите имя файла: ");
    io::stdout().flush().unwrap();

    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    let content = fs::read_to_string(filename)
        .map_err(|_| format!("Ошибка, файл \"{}\" не найден!", filename))?;

    let mut all_numbers = Vec::new();
    for word in content.split_whitespace() {
        let clean_word = word.replace(",", ".");
        let num: f64 = clean_word
            .parse()
            .map_err(|_| format!("Ошибка: '{}' не является числом!", clean_word))?;
        all_numbers.push(num);
    }

    if all_numbers.len() != (all_numbers[0] * all_numbers[0] + all_numbers[0] + 2.0) as usize {
        return Err("Ошибка формата файла".to_string());
    }
    
    let mut current_pos = 0;
    
    let n = all_numbers[current_pos] as usize;
    current_pos += 1;

    let mut a = vec![vec![0.0; n]; n];
    let mut b = vec![0.0; n];
    
    for i in 0..n {
        for j in 0..n {
            a[i][j] = all_numbers[current_pos];
            current_pos += 1;
        }
        // свободный член вектора B
        b[i] = all_numbers[current_pos];
        current_pos += 1;
    }
    
    let epsilon = all_numbers[current_pos];

    Ok((a, b, epsilon))
}

pub fn read_matrix_from_keyboard() -> Result<(Vec<Vec<f64>>, Vec<f64>, f64), String> {
    print!("Введите размерность матрицы n (до 20): ");
    io::stdout().flush().unwrap();
    let mut n_str = String::new();
    io::stdin()
        .read_line(&mut n_str)
        .map_err(|e| e.to_string())?;
    let n: usize = n_str
        .trim()
        .parse()
        .map_err(|_| "Размерность должна быть целым числом")?;

    if n == 0 || n > 20 {
        return Err("Размерность должна быть от 1 до 20".to_string());
    }

    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);

    println!(
        "Введите строки матрицы. Каждая строка должна содержать чисел: {} ({} - коэффициенты A и затем b):",
        n + 1,
        n
    );

    for i in 0..n {
        loop {
            print!("Строка {}: ", i + 1);
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .map_err(|e| e.to_string())?;

            let nums: Vec<f64> = line
                .split_whitespace()
                .map(|s| s.replace(',', ".").parse())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "Ошибка: введите корректные числа через пробел")?;

            if nums.len() == n + 1 {
                let mut row = nums;
                b.push(row.pop().unwrap()); // последнее число в b
                a.push(row);
                break; // выход
            } else {
                println!(
                    "ОШИБКА: Нужно ввести ровно {} чисел. Попробуйте еще раз.",
                    n + 1
                );
            }
        }
    }
    
    print!("Введите точность (epsilon, например 0.001): ");
    io::stdout().flush().unwrap();
    let mut eps_str = String::new();
    io::stdin()
        .read_line(&mut eps_str)
        .map_err(|e| e.to_string())?;
    let epsilon: f64 = eps_str
        .trim()
        .replace(',', ".")
        .parse()
        .map_err(|_| "Точность должна быть числом")?;

    Ok((a, b, epsilon))
}
pub fn gen_random_matrix() -> Result<(Vec<Vec<f64>>, Vec<f64>, f64), String> {
    print!("Введите размерность матрицы n (до 20): ");
    io::stdout().flush().unwrap();
    let mut n_str = String::new();
    io::stdin()
        .read_line(&mut n_str)
        .map_err(|e| e.to_string())?;
    let n: usize = n_str
        .trim()
        .parse()
        .map_err(|_| "Размерность должна быть целым числом")?;

    if n == 0 || n > 20 {
        return Err("Размерность должна быть от 1 до 20".to_string());
    }

    let mut rng = rand::thread_rng();
    let mut a = vec![vec![0.0; n]; n];
    let mut b = vec![0.0; n];

    for i in 0..n {
        let mut row_sum = 0.0;
        
        for j in 0..n {
            if i != j {
                let val = rng.gen_range(-10.0..10.0);
                a[i][j] = val;
                row_sum += if val < 0.0 { -val } else { val }; // Считаем сумму модулей
            }
        }

        // генерируем диагональный
        let diagonal_val = row_sum + rng.gen_range(1.0..10.0);

        // по приколу знак поменяем
        a[i][i] = if rng.gen_bool(0.5) {
            diagonal_val
        } else {
            -diagonal_val
        };

        b[i] = rng.gen_range(-10.0..10.0);
    }

    print("Матрица успешно сгенерирована:", Left);

    print_matrix(&a, &b, 2);

    print!("Введите точность (epsilon, например 0.001): ");
    io::stdout().flush().unwrap();
    let mut eps_str = String::new();
    io::stdin()
        .read_line(&mut eps_str)
        .map_err(|e| e.to_string())?;
    let epsilon: f64 = eps_str
        .trim()
        .replace(',', ".")
        .parse()
        .map_err(|_| "Точность должна быть числом")?;

    Ok((a, b, epsilon))
}
