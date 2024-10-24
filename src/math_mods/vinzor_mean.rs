use crate::{math_mods, small_logic};
use std::io;

pub fn count() -> i8 {
    let mut user_input = String::new();
    println!("Введите процент усечения: ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Не удалось прочитать");
    let trim_percent: f64 = user_input
        .trim()
        .parse()
        .expect("Неверный тип входных данных");
    if trim_percent < 0.0 || trim_percent > 1.0 {
        panic!(
            "Невалидный тип данных: число должно быть в промежутке 
            от 0.0 до 1.0"
        );
    }

    let mut result = small_logic::get_user_i128_input();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut result_f64: Vec<f64> = Vec::new();
    for i in 0..result.len() {
        result_f64.push(result[i] as f64);
    }
    let vinz_count = (result_f64.len() as f64 * trim_percent).round() as usize;
    let mut vinz_result: Vec<f64> = vec![];
    result_f64[vinz_count..result_f64.len() - vinz_count].clone_into(&mut vinz_result);

    for i in 0..vinz_count {
        result_f64[i] = vinz_result[0];
    }
    for i in result_f64.len() - vinz_count..result_f64.len() {
        result_f64[i] = vinz_result[vinz_result.len() - 1];
    }

    let answer: f64 = result_f64.iter().sum::<f64>() / result_f64.len() as f64;
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}
