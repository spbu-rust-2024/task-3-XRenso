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
/// for i in 0..result_user_input.len() {
///     multiple_all_numbers *= result_user_input[i];
/// }
/// let answer: f64 = f64::powf(multiple_all_numbers as f64, 1.0 / result_user_input.len() as f64);
/// ```
///

pub fn count(num_array: Vec<i128>) -> f64 {
    let result_user_input: Vec<i128>;
    if num_array.is_empty() {
        result_user_input = small_logic::get_user_i128_input();
    } else {
        result_user_input = num_array;
    }
    let mut multiple_all_numbers: i128 = 1;
    for i in 0..result_user_input.len() {
        multiple_all_numbers *= result_user_input[i];
    }
    let answer: f64 = f64::powf(
        multiple_all_numbers as f64,
        1.0 / result_user_input.len() as f64,
    );
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
    fn count_geometry_mean_1() {
        let input: Vec<i128> = vec![1, 1, 2, 3, 4, 5];
        assert_eq!(count(input), 2.220906154852325)
    }

    #[test]
    fn count_dispersion_2() {
        let input: Vec<i128> = vec![1, 1, 1, 1, 1, 1, 1];
        assert_eq!(count(input), 1.0)
    }
}
