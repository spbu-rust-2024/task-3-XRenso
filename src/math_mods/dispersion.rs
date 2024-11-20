use crate::{math_mods, small_logic};

/// Функция для расчета дисперсии
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Мы находим среднее арефметическое всех чисел и записываем в переменную
/// `u` типа `f64`
/// ```
/// let sum_all_numbers_in_input = result_user_input_user_input.iter().sum::<i128>();
/// let u: f64 = sum_all_numbers_in_input as f64 / result_user_input_user_input.len() as f64;
/// ```
/// Заполняем массив из чисел, что будут представлять из себя разницу каждого
/// числа
/// со средним арифметическим в квадрате
/// ```
/// let mut x_i: Vec<f64> = Vec::new();
/// for i in result_user_input_user_input.iter() {
///     x_i.push(f64::powf(*i as f64 - u, 2.0));
/// }
/// ```
/// Результатом же будет среднее этих чисел
///```
/// let answer: f64 = x_i.iter().sum::<f64>() / x_i.len() as f64;
/// ```
///

pub fn count(num_array: Vec<i128>) -> f64 {
    let result_user_input: Vec<i128>;
    if num_array.is_empty() {
        result_user_input = small_logic::get_user_i128_input();
    } else {
        result_user_input = num_array;
    }
    let sum_all_numbers_in_input = result_user_input.iter().sum::<i128>();
    let u: f64 = sum_all_numbers_in_input as f64 / result_user_input.len() as f64;
    let mut x_i: Vec<f64> = Vec::new();
    for i in result_user_input.iter() {
        x_i.push(f64::powf(*i as f64 - u, 2.0));
    }
    let answer: f64 = x_i.iter().sum::<f64>() / x_i.len() as f64;
    answer
}

pub fn print_res(num_array: Vec<i128>) -> i8 {
    let answer = count(num_array);
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_dispersion_1() {
        let input: Vec<i128> = vec![1, 2, 3, 4, 5];
        assert_eq!(count(input), 2.0)
    }

    #[test]
    fn count_dispersion_2() {
        let input: Vec<i128> = vec![1, 1, 1, 1, 1, 1, 1];
        assert_eq!(count(input), 0.0)
    }
}
