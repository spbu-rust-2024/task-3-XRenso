use crate::{math_mods, small_logic};
use std::io;
pub fn count() -> i8 {
    let mut user_input = String::new();
    println!("Введите глубину расчета: ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Не удалось прочитать");
    let deep: i128 = user_input
        .trim()
        .parse()
        .expect("Неверный тип входных данных");

    if deep < 1 {
        panic!("Невалидные входные данные: Глубина расчетов меньше 1");
    }
    let result = small_logic::get_user_i128_input();
    if result.len() != 2 {
        panic!("Невалидные входные данные: должно быть 2 числа");
    }
    let mut a = result[0] as f64;
    let mut b = result[1] as f64;
    let mut a0 = a;
    let mut b0 = b;

    let mut a_vec: Vec<f64> = Vec::new();
    let mut b_vec: Vec<f64> = Vec::new();

    for _ in 0..deep {
        a_vec.push(a0);
        b_vec.push(b0);
        a = (a0 + b0) / 2.0;
        b = f64::powf(a0 * b0, 1.0 / 2.0);
        a0 = a;
        b0 = b;
    }
    a_vec.push(a0);
    b_vec.push(b0);

    println!(
        "Ваш ответ\n\nАрефметическое: {:?}\nГеометрическое: {:?}",
        a_vec, b_vec
    );
    return math_mods::exit_code_algos();
}
