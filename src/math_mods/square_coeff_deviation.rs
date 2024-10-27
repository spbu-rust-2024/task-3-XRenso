use crate::{math_mods, small_logic};

/// Функция для расчета квадратического коэффициент вариации
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Мы находим среднее арефметическое всех чисел и записываем в перменную `u`
/// ```
/// let sum_all_numbers_in_input = result.iter().sum::<i128>();
/// let u: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
/// ```
/// После мы создаем вектор `x_i` и заполняем его квадратом разницы чисел и среднего арифметического
/// ```
/// let mut x_i: Vec<f64> = Vec::new();
/// for i in result.iter() {
///         x_i.push(f64::powf(*i as f64 - u, 2.0));
/// }
/// ```
/// Ответом же будет являться корень квадратный из среднего арифметического
/// `x_i` деленного на `u`
/// ```
/// let mut answer: f64 = f64::powf(x_i.iter().sum::<f64>() / x_i.len() as f64, 1.0 / 2.0);
/// answer /= u;
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
    let mut answer: f64 = f64::powf(x_i.iter().sum::<f64>() / x_i.len() as f64, 1.0 / 2.0);
    answer /= u;
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}
