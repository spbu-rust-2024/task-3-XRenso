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
    let mut x = result[0] as f64;
    let mut y = result[1] as f64;
    let mut x0 = x;
    let mut y0 = y;
    let mut z0: f64 = 0.0;
    // let mut z = z0;
    let mut x_vec: Vec<f64> = Vec::new();
    let mut y_vec: Vec<f64> = Vec::new();

    for _ in 0..deep {
        x_vec.push(x0);
        y_vec.push(y0);

        x = (x0 + y0) / 2.0;
        y = z0 + f64::powf((x0 - z0) * (y0 - z0), 1.0 / 2.0);
        z0 = z0 - f64::powf((x0 - z0) * (y0 - z0), 1.0 / 2.0);
        x0 = x;
        y0 = y;
    }
    x_vec.push(x0);
    y_vec.push(y0);

    println!(
        "Ваш ответ\n\nАрефметическое: {:?}\nГеометрическое: {:?}",
        x_vec, y_vec
    );
    return math_mods::exit_code_algos();
}
