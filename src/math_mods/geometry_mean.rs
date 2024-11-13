use crate::{math_mods, small_logic};

/// Функция для расчета геометрического среднего
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Мы пробегаемся по всему вектору и умножаем числа друг на друга,
/// после чего извлекаем корень равный длине вектора
///```
/// let mut multiple_all_numbers: i128 = 1;
/// for i in 0..result.len() {
///     multiple_all_numbers *= result[i];
/// }
/// let answer: f64 = f64::powf(multiple_all_numbers as f64, 1.0 / result.len() as f64);
/// ```
///

pub fn print_res(num_array: Vec<i128>) -> i8 {
    let result: Vec<i128>;
    if num_array.is_empty() {
        result = small_logic::get_user_i128_input();
    } else {
        result = num_array;
    }
    let mut multiple_all_numbers: i128 = 1;
    for i in 0..result.len() {
        multiple_all_numbers *= result[i];
    }
    let answer: f64 = f64::powf(multiple_all_numbers as f64, 1.0 / result.len() as f64);
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}
