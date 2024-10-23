use crate::{math_mods, small_logic};

// При f(x)=x
pub fn count() -> i8 {
    let result = small_logic::get_user_i128_input();
    let sum_all_numbers_in_input = result.iter().sum::<i128>();
    let answer: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}
