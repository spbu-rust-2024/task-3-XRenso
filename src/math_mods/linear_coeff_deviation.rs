use crate::{math_mods, small_logic};

/// Функция для расчета линейного коэффициента вариации
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Находим среднее арифметическое всех чисел и записываем в переменну. `x_e`
/// ```
/// let sum_all_numbers_in_input = result.iter().sum::<i128>();
///
/// let x_e: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
/// ```
/// После мы находим среднее арифметическое из чисел, что равны разнице каждого
/// элемента с средним арифметическим.
/// После чего делим на среднее арифметическое
/// ```
/// let mut x_i: Vec<f64> = Vec::new();
///     for i in result.iter() {
///         x_i.push((*i as f64 - x_e).abs());
/// }
/// let mut d: f64 = x_i.iter().sum::<f64>();
/// d /= x_i.len() as f64;
///
/// d /= x_e;
/// ```

pub fn count(num_array: Vec<i128>) -> i8 {
    let result: Vec<i128>;
    if num_array.is_empty() {
        result = small_logic::get_user_i128_input();
    } else {
        result = num_array;
    }
    let sum_all_numbers_in_input = result.iter().sum::<i128>();

    let x_e: f64 = sum_all_numbers_in_input as f64 / result.len() as f64;
    let mut x_i: Vec<f64> = Vec::new();
    for i in result.iter() {
        x_i.push((*i as f64 - x_e).abs());
    }
    let mut d: f64 = x_i.iter().sum::<f64>();
    d /= x_i.len() as f64;

    d /= x_e;
    println!("Ваш результат: {}", d);
    return math_mods::exit_code_algos();
}
