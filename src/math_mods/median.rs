use crate::{math_mods, small_logic};

/// Функция для расчета медианы
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Мы сортируем вектор
/// ```
/// result.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
/// ```
/// Если наш вектор четный, то медианой будет являеться среднее двух чисел в центре вектора
/// ```
/// if result.len() % 2 == 0 {
///     answer = (result[result.len() / 2 - 1] as f64 + result[result.len() / 2] as f64) / 2.0;
/// }
/// ```
/// Иначе же это просто число в середине вектора
/// ```
/// else {
///     let idx = result.len() as f64 / 2.0;
///     answer = result[idx.round() as usize - 1] as f64;
/// }
/// ```

pub fn print_res(num_array: Vec<i128>) -> i8 {
    let mut result: Vec<i128>;
    if num_array.is_empty() {
        result = small_logic::get_user_i128_input();
    } else {
        result = num_array;
    }
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
