use crate::{math_mods, small_logic};

/// Функция для расчета медианы
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Мы сортируем вектор
/// ```
/// result_user_input.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
/// ```
/// Если наш вектор четный, то медианой будет являеться среднее двух чисел в центре вектора
/// ```
/// if result_user_input.len() % 2 == 0 {
///     answer = (result_user_input[result_user_input.len() / 2 - 1] as f64 + result_user_input[result_user_input.len() / 2] as f64) / 2.0;
/// }
/// ```
/// Иначе же это просто число в середине вектора
/// ```
/// else {
///     let idx = result_user_input.len() as f64 / 2.0;
///     answer = result_user_input[idx.round() as usize - 1] as f64;
/// }
/// ```
pub fn count(num_array: Vec<i128>) -> f64 {
    let mut result_user_input: Vec<i128>;
    if num_array.is_empty() {
        result_user_input = small_logic::get_user_i128_input();
    } else {
        result_user_input = num_array;
    }
    result_user_input.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let answer: f64;
    if result_user_input.len() % 2 == 0 {
        answer = (result_user_input[result_user_input.len() / 2 - 1] as f64
            + result_user_input[result_user_input.len() / 2] as f64)
            / 2.0;
    } else {
        let idx = result_user_input.len() as f64 / 2.0;
        answer = result_user_input[idx.round() as usize - 1] as f64;
    }
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
    fn median_same_number() {
        let input: Vec<i128> = vec![1, 1, 1, 1, 1, 1];
        assert_eq!(count(input), 1.0)
    }

    #[test]
    fn median_even_count_numbers() {
        let input: Vec<i128> = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(count(input), 3.5)
    }

    #[test]
    fn median_odd_count_numbers() {
        let input: Vec<i128> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(count(input), 4.0)
    }
}
