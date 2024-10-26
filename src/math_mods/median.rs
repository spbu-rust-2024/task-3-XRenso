use crate::{math_mods, small_logic};

pub fn count() -> i8 {
    let mut result = small_logic::get_user_i128_input();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let answer: f64;
    if result.len() % 2 == 0 {
        answer = (result[result.len() / 2 - 1] as f64 + result[result.len() / 2] as f64) / 2.0;
    } else {
        let idx = result.len() as f64 / 2.0;
        answer = result[idx.round() as usize - 1] as f64;
    }
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}
