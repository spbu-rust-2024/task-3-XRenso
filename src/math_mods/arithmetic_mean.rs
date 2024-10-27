use crate::{math_mods, small_logic};

/// Функция для расчета арифметического среднего
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Мы пробегаемся по всему вектору и складываем числа, после чего делим
/// на длину вектора
///```
/// let sum_all_numbers_in_input = result.iter().sum::<i128>();
/// let answer: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
/// ```

pub fn count(num_array: Vec<i128>) -> i8 {
    let result: Vec<i128>;
    if num_array.is_empty() {
        result = small_logic::get_user_i128_input();
    } else {
        result = num_array;
    }
    let sum_all_numbers_in_input = result.iter().sum::<i128>();
    let answer: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}
