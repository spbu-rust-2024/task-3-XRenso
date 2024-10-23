use crate::{math_mods, small_logic};
use std::io;
pub fn count() -> i8 {
    let mut user_input = String::new();
    println!("Введите степень усреднения: ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Не удалось прочитать");
    let degree: i128 = user_input
        .trim()
        .parse()
        .expect("Неверный тип входных данных");
    let result = small_logic::get_user_i128_input();
    let mut ans: f64 = 0.0;
    for i in 0..result.len() {
        ans += f64::powf(result[i] as f64, degree as f64);
    }
    ans /= result.len() as f64;
    ans = ans.powf(1.0 / degree as f64);
    println!("Ваш результат: {}", ans);
    return math_mods::exit_code_algos();
}
