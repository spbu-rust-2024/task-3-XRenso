use crate::{math_mods, small_logic};

/// Функция для расчета среднего линейного отклонения
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Мы ищем среднее арифметическое всех чисел из пользовательского ввода
///  в переменную `x_e`
/// ```
/// let sum_all_numbers_in_input = result.iter().sum::<i128>();
/// let x_e: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
/// ```
/// После же для каждого числа ищем модуль разница со средним и записываем в
/// вектор `x_i`
/// ```
/// let mut x_i: Vec<f64> = Vec::new();
/// for i in result.iter() {
///     x_i.push((*i as f64 - x_e).abs());
/// }
/// ```
/// Ответом же будет среднее арифметическое этих разниц
/// ```
/// let mut answer: f64 = x_i.iter().sum::<f64>();
/// answer /= x_i.len() as f64;
/// ```
pub fn count(num_array: Vec<i128>) -> f64 {
    let result_user_input: Vec<i128>;
    if num_array.is_empty() {
        result_user_input = small_logic::get_user_i128_input();
    } else {
        result_user_input = num_array;
    }
    let sum_all_numbers_in_input = result_user_input.iter().sum::<i128>();
    let x_e: f64 = sum_all_numbers_in_input as f64 / result_user_input.len() as f64;
    let mut x_i: Vec<f64> = Vec::new();
    for i in result_user_input.iter() {
        x_i.push((*i as f64 - x_e).abs());
    }
    let mut answer: f64 = x_i.iter().sum::<f64>();
    answer /= x_i.len() as f64;
    answer
}

pub fn print_res(num_array: Vec<i128>) -> i8 {
    let answer = count(num_array);
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}
