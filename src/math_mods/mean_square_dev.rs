use crate::{math_mods, small_logic};

/// Функция для расчета среднего квадратического отклонения
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Ищем средее арифметическое и записываем в перменную `u`
/// ```
/// let sum_all_numbers_in_input = result.iter().sum::<i128>();
/// let u: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
/// ```
/// Записываем квадрат разницы в вектор `x_i`
/// каждого числа со среднем арифметическим
/// ```
/// let mut x_i: Vec<f64> = Vec::new();
/// for i in result.iter() {
///     x_i.push(f64::powf(*i as f64 - u, 2.0));
/// }
/// ```
/// результатом будет квадратный корень из среднего арифметического вектора
/// `x_i`
/// ```
/// let answer: f64 = f64::powf(x_i.iter().sum::<f64>() / x_i.len() as f64, 1.0 / 2.0);
/// ```

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
