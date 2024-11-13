use crate::{math_mods, small_logic};
use std::f64::consts::E;

/// Функция для расчета φ
/// # Входные данные
/// * `x` - число с палвающей точкой типа f64, значение которые мы хотим преобразовать
///
/// # Принцип работы
/// f(x) = (e^x) + 2 * (x^2) + x + 5
fn f(x: f64) -> f64 {
    E.powf(x) + 2.0 * x.powf(2.0) + x + 5.0
}

/// Функция для расчета производной f(x)
/// # Входные данные
/// * `x` - число с палвающей точкой типа f64, значение которые мы хотим преобразовать
///
/// # Принцип работы
/// f_prime(x) = e^x + 4*x + 1

fn f_prime(x: f64) -> f64 {
    E.powf(x) + 4.0 * x + 1.0
}

/// Функция для расчета обратной от f(x)
/// # Входные данные
/// * `target_value` - число с палвающей точкой типа f64, значение от которого мы ищем обратное
/// * `x0_init` - начальное приближение
/// * `tolerance` - точность найденного числа
/// * `max_iter` - максимальная длина цикла
///
/// # Принцип работы
/// Мы ищем производную и
/// разницу между f(x) от изначального приближения и `target_value`
///```
/// let mut x = x0_init;
/// let f_x = f(x) - target_value;
/// let f_x_prime = f_prime(x);
/// ```
///
/// Если производная очень маленькая, то наш метод не будет работать
/// ```
/// if f_x_prime.abs() < tolerance {
///     println!("Малая производная. Метод не сходится");
///     return 1.0;
/// }
/// ```
///
/// После чего создаем перменную `x_new` и сравниваем её разность с x.
/// Если эта разница меньше нашей точности, то это и будет ответом. Иначе
/// мы будем продолжать работу
/// ```
/// x_new = x - f_x / f_x_prime;
/// if (x_new - x).abs() < tolerance {
///     return x_new;
/// }
/// x = x_new;
/// ```
fn f_reverse(target_value: f64, x0_init: f64, tolerance: f64, max_iter: u16) -> f64 {
    let mut x = x0_init;
    let mut x_new: f64 = 0.0;
    for _ in 0..max_iter {
        let f_x = f(x) - target_value;
        let f_x_prime = f_prime(x);
        if f_x_prime.abs() < tolerance {
            println!("Малая производная. Метод не сходится");
            return 1.0;
        }
        x_new = x - f_x / f_x_prime;
        if (x_new - x).abs() < tolerance {
            return x_new;
        }
        x = x_new;
    }
    x_new
}

/// Функция для расчета среднего Колмогорова
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Мы пробегаемся по всему вектору и применяем функцию `f(x)`, делим на
/// количество чисел и находим обратное значение в функии `f_reverse(x)`
/// ```
/// let mut x: f64 = 0.0;
/// for i in result_user_input.iter() {
///     x += f(*i as f64);
/// }
/// x /= result_user_input.len() as f64;
/// let ans = f_reverse(x, 0.0, 1e-6, 9999);
/// ```
pub fn count(num_array: Vec<i128>) -> f64 {
    let result_user_input: Vec<i128>;
    if num_array.is_empty() {
        result_user_input = small_logic::get_user_i128_input();
    } else {
        result_user_input = num_array;
    }
    let mut x: f64 = 0.0;
    for i in result_user_input.iter() {
        x += f(*i as f64);
    }
    x /= result_user_input.len() as f64;
    let ans = f_reverse(x, 0.0, 1e-6, 9999);
    ans
}
pub fn print_res(num_array: Vec<i128>) -> i8 {
    let answer = count(num_array);
    println!("Ваш ответ: {}", answer);
    return math_mods::exit_code_algos();
}
