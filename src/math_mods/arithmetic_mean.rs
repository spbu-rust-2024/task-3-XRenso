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
/// let answer: f64 = sum_all_numbers_in_input as f64 / result_user_input.len() as f64;
/// ```

pub fn count(num_array: Vec<i128>) -> f64 {
    let result_user_input: Vec<i128>;
    if num_array.is_empty() {
        result_user_input = small_logic::get_user_i128_input();
    } else {
        result_user_input = num_array;
    }
    let sum_all_numbers_in_input = result_user_input.iter().sum::<i128>();
    let answer: f64 = sum_all_numbers_in_input as f64 / result_user_input.len() as f64;
    return answer;
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
    fn count_arth_mean_from_one_element() {
        let input: Vec<i128> = vec![1];
        assert_eq!(count(input), 1.0);
    }
    #[test]
    fn count_arth_mean_with_many_args() {
        let input: Vec<i128> = vec![10, 10, 5, 6, 7, 8, 19, 20];
        assert_eq!(count(input), 10.625);
    }
}
