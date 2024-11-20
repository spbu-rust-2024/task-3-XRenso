use crate::{math_mods, small_logic};
use std::io;

/// Функция для расчета винсоризованного среднего
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
/// * `depth` - число с плавающей точкой типа `Option<f64>`, которое
/// испсользуется для процента усечения
///
/// # Принцип работы
/// Сортируем вектор чисел и согласно проценту усечения заменяем числа
/// ```
/// result_user_input.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
/// let mut result_user_input_f64: Vec<f64> = Vec::new();
/// for i in 0..result_user_input.len() {
///     result_user_input_f64.push(result_user_input[i] as f64);
/// }
/// let vinz_count = (result_user_input_f64.len() as f64 * trim_percent).round() as usize;
/// let mut vinz_result_user_input: Vec<f64> = vec![];
/// result_user_input_f64[vinz_count..result_user_input_f64.len() - vinz_count].clone_into(&mut vinz_result_user_input);
/// for i in 0..vinz_count {
///     result_user_input_f64[i] = vinz_result_user_input[0];
/// }
/// for i in result_user_input_f64.len() - vinz_count..result_user_input_f64.len() {
///     result_user_input_f64[i] = vinz_result_user_input[vinz_result_user_input.len() - 1];
/// }
/// ```
/// Ответом будет среднее арефметическое этого вектора
/// ```
/// let answer: f64 = result_user_input_f64.iter().sum::<f64>() / result_user_input_f64.len() as f64;
/// ```

pub fn count(num_array: Vec<i128>, depth: Option<f64>) -> f64 {
    let trim_percent: f64;
    let mut result_user_input: Vec<i128>;
    if depth == None {
        let mut user_input = String::new();
        println!("Введите процент усечения: ");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Не удалось прочитать");
        trim_percent = user_input
            .trim()
            .parse()
            .expect("Неверный тип входных данных");
    } else {
        trim_percent = depth.unwrap();
    }
    if trim_percent < 0.0 || trim_percent > 1.0 {
        panic!(
            "Невалидный тип данных: число должно быть в промежутке 
            от 0.0 до 1.0"
        );
    }
    if num_array.is_empty() {
        result_user_input = small_logic::get_user_i128_input();
    } else {
        result_user_input = num_array;
    }
    result_user_input.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut result_user_input_f64: Vec<f64> = Vec::new();
    for i in 0..result_user_input.len() {
        result_user_input_f64.push(result_user_input[i] as f64);
    }
    let vinz_count = (result_user_input_f64.len() as f64 * trim_percent).round() as usize;
    let mut vinz_result_user_input: Vec<f64> = vec![];
    result_user_input_f64[vinz_count..result_user_input_f64.len() - vinz_count]
        .clone_into(&mut vinz_result_user_input);

    for i in 0..vinz_count {
        result_user_input_f64[i] = vinz_result_user_input[0];
    }
    for i in result_user_input_f64.len() - vinz_count..result_user_input_f64.len() {
        result_user_input_f64[i] = vinz_result_user_input[vinz_result_user_input.len() - 1];
    }

    let answer: f64 =
        result_user_input_f64.iter().sum::<f64>() / result_user_input_f64.len() as f64;
    answer
}

pub fn print_res(num_array: Vec<i128>, depth: Option<f64>) -> i8 {
    let answer = count(num_array, depth);
    println!("Ваш результат: {}", answer);
    return math_mods::exit_code_algos();
}

mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn vinzor_mean_error_wrong_depth() {
        let input: Vec<i128> = vec![1, 1, 1, 1, 1, 1];
        count(input, Some(2.0));
    }
    #[test]
    fn vinzor_mean_1() {
        let input: Vec<i128> = vec![1, 1, 1, 1, 1];
        assert_eq!(count(input, Some(0.2)), 1.0)
    }

    #[test]
    fn vinzor_mean_2() {
        let input: Vec<i128> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(count(input, Some(0.3)), 4.0)
    }
}
