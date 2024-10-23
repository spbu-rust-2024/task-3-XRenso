use crate::{math_mods, small_logic};

pub fn count() -> i8 {
    let result = small_logic::get_user_i128_input();
    let mut multiple_all_numbers: i128 = 1;
    for i in 0..result.len() {
        multiple_all_numbers *= result[i];
    }
    let answer: f64 = f64::powf(multiple_all_numbers as f64, 1.0 / result.len() as f64);
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}
