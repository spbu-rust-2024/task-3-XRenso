use crate::{math_mods, small_logic};

pub fn count(num_array: Vec<i128>) -> i8 {
    let result: Vec<i128>;
    if num_array.is_empty() {
        result = small_logic::get_user_i128_input();
    } else {
        result = num_array;
    }
    let sum_all_numbers_in_input = result.iter().sum::<i128>();
    let u: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
    let mut x_i: Vec<f64> = Vec::new();
    for i in result.iter() {
        x_i.push(f64::powf(*i as f64 - u, 2.0));
    }
    let answer: f64 = f64::powf(x_i.iter().sum::<f64>() / x_i.len() as f64, 1.0 / 2.0);
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}
