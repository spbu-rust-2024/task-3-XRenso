use crate::math_mods;
use std::f64::consts::E;
fn f(x: f64) -> f64 {
    let ans = f64::powf(E, x) + 2.0 * f64::powf(x, 2.0) + x + 5.0;
    return ans;
}
// (sqrt((-8*%e^x)-39)-1)/4
fn f_reverse(x: f64) -> f64 {
    let ans = (f64::powf((-8.0 * E.powf(x)) - 39.0, 1.0 / 2.0) - 1.0) / 4.0;
    return ans;
}
pub fn count() -> i8 {
    // let result = small_logic::get_user_i128_input();
    // let sum_all_numbers_in_input = result.iter().sum::<i128>();
    // let answer: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
    let x = f(1.0);
    let y = f_reverse(x);
    println!("Ваш xc: {}\nВаш у: {}", x, y);
    return math_mods::exit_code_algos();
}
