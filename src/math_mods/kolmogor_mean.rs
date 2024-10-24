use crate::{math_mods, small_logic};
use std::f64::consts::E;
fn f(x: f64) -> f64 {
    f64::powf(E, x) + 2.0 * f64::powf(x, 2.0) + x + 5.0
}

fn f_prime(x: f64) -> f64 {
    E.powf(x) + 4.0 * x + 1.0
}

fn f_reverse(target_value: f64, x0_init: f64, tolerance: f64, max_iter: u16) -> f64 {
    let mut x = x0_init;
    let mut x_new: f64 = 0.0;
    for _ in 0..max_iter {
        let f_x = f(x) - target_value;
        let f_x_prime = f_prime(x);
        if f_x_prime.abs() < tolerance {
            println!("Малая производная. Метод не сходится");
            return 1.0;
        }
        x_new = x - f_x / f_x_prime;
        if (x_new - x).abs() < tolerance {
            return x_new;
        }
        x = x_new;
    }
    x_new
}

pub fn count() -> i8 {
    let result = small_logic::get_user_i128_input();
    let mut x: f64 = 0.0;
    for i in result.iter() {
        x += f(*i as f64);
    }
    x /= result.len() as f64;
    let ans = f_reverse(x, 0.0, 1e-6, 9999);
    println!("Ваш ответ: {}", ans);
    return math_mods::exit_code_algos();
}
