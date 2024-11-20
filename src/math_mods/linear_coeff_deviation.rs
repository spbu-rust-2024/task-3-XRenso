use crate::{math_mods, small_logic};

/// Функция для расчета линейного коэффициента вариации
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Находим среднее арифметическое всех чисел и записываем в переменну. `x_e`
/// ```
/// let sum_all_numbers_in_input = result_user_input.iter().sum::<i128>();
///
/// let x_e: f64 = sum_all_numbers_in_input as f64 / result_user_input.len() as f64;
/// ```
/// После мы находим среднее арифметическое из чисел, что равны разнице каждого
/// элемента с средним арифметическим.
/// После чего делим на среднее арифметическое
/// ```
/// let mut x_i: Vec<f64> = Vec::new();
///     for i in result_user_input.iter() {
///         x_i.push((*i as f64 - x_e).abs());
/// }
/// let mut d: f64 = x_i.iter().sum::<f64>();
/// d /= x_i.len() as f64;
///
/// d /= x_e;
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
    let mut d: f64 = x_i.iter().sum::<f64>();
    d /= x_i.len() as f64;

    d /= x_e;
    d
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
    fn linnear_coeff_count_1() {
        let input: Vec<i128> = vec![1, 1, 2, 3, 4, 5];
        assert_eq!(count(input), 2.220906154852325)
    }

    #[test]
    fn count_dispersion_2() {
        let input: Vec<i128> = vec![1, 1, 1, 1, 1, 1, 1];
        assert_eq!(count(input), 1.0)
    }
}
