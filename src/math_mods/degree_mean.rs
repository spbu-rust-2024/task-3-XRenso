use crate::{math_mods, small_logic};
use std::io;

/// Функция для расчета степенного среднего
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
/// * `depth` - число с плавающей точкой типа `Option<f64>`, которое мы
/// используем для степени, по которой идет усреднение
///
/// # Принцип работы
/// Мы возводим все числа в степень и находим их сумму
/// ```
/// let mut ans: f64 = 0.0;
/// for i in 0..result.len() {
///     ans += f64::powf(result[i] as f64, degree as f64);
/// }
/// ```
/// После делим эту сумму на число элементов и извлекаем корень
/// степени усреднения
/// ```
/// ans /= result.len() as f64;
/// ans = ans.powf(1.0 / degree as f64);
/// ```

pub fn print_res(num_array: Vec<i128>, depth: Option<f64>) -> i8 {
    let degree: i128;
    let result: Vec<i128>;
    if depth == None {
        let mut user_input = String::new();
        println!("Введите степень усреднения: ");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Не удалось прочитать");
        degree = user_input
            .trim()
            .parse()
            .expect("Неверный тип входных данных");
    } else {
        degree = depth.unwrap() as i128;
    }
    if num_array.is_empty() {
        result = small_logic::get_user_i128_input();
    } else {
        result = num_array;
    }

    let mut ans: f64 = 0.0;
    for i in 0..result.len() {
        ans += f64::powf(result[i] as f64, degree as f64);
    }
    ans /= result.len() as f64;
    ans = ans.powf(1.0 / degree as f64);
    println!("Ваш результат: {}", ans);
    return math_mods::exit_code_algos();
}
