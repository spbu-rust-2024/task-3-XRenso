use crate::{math_mods, small_logic};

pub fn count() -> i8 {
    let result = small_logic::get_user_i128_input();
    let sum_all_numbers_in_input = result.iter().sum::<i128>();
    let x_e: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
    let mut x_i: Vec<f64> = Vec::new();
    for i in result.iter() {
        x_i.push(*i as f64 - x_e);
    }
    let mut answer: f64 = x_i.iter().sum::<f64>();
    answer /= x_i.len() as f64;
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}