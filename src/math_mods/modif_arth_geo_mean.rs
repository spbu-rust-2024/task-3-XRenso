use crate::{math_mods, small_logic};
use std::io;

/// Функция для расчета Модифицированное арефметико-геометрического среднего
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов.
/// Должен состоять лишь из двух чисел
/// * `depth` - число с плавающей точкой типа `f64`,
/// которое мы используем для глубины расчетов. Получается при помощи флага
/// из консоли
/// # Принцип работы
/// Задаем стартовые параметры `x`, `y` и `z`. Также создаем векторы для сохранения последовательности
/// ```
/// let mut x = result_user_input[0] as f64;
/// let mut y = result_user_input[1] as f64;
/// let mut x0 = x;
/// let mut y0 = y;
/// let mut z0: f64 = 0.0;
/// let mut x_vec: Vec<f64> = Vec::new();
/// let mut y_vec: Vec<f64> = Vec::new();
/// ```
/// После мы заполняем наши последовательности через цикл длины depth при помощи
/// мат. формул для данной средней
/// ```
/// for _ in 0..deep {
///     x_vec.push(x0);
///     y_vec.push(y0);
///
///     x = (x0 + y0) / 2.0;
///     y = z0 + f64::powf((x0 - z0) * (y0 - z0), 1.0 / 2.0);
///     z0 = z0 - f64::powf((x0 - z0) * (y0 - z0), 1.0 / 2.0);
///     x0 = x;
///     y0 = y;
/// }
/// x_vec.push(x0);
/// y_vec.push(y0);
/// ```

pub fn count(num_array: Vec<i128>, depth: Option<f64>) -> Vec<Vec<f64>> {
    let deep: i128;
    let result_user_input: Vec<i128>;
    if depth == None {
        let mut user_input = String::new();
        println!("Введите глубину расчета: ");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Не удалось прочитать");
        deep = user_input
            .trim()
            .parse()
            .expect("Неверный тип входных данных");

        if deep < 1 {
            panic!("Невалидные входные данные: Глубина расчетов меньше 1");
        }
    } else {
        deep = depth.unwrap() as i128;
    }
    if num_array.is_empty() {
        result_user_input = small_logic::get_user_i128_input();
    } else {
        result_user_input = num_array;
    }
    if result_user_input.len() != 2 {
        panic!("Невалидные входные данные: должно быть 2 числа");
    }
    let mut x = result_user_input[0] as f64;
    let mut y = result_user_input[1] as f64;
    let mut x0 = x;
    let mut y0 = y;
    let mut z0: f64 = 0.0;
    let mut x_vec: Vec<f64> = Vec::new();
    let mut y_vec: Vec<f64> = Vec::new();

    for _ in 0..deep {
        x_vec.push(x0);
        y_vec.push(y0);

        x = (x0 + y0) / 2.0;
        y = z0 + f64::powf((x0 - z0) * (y0 - z0), 1.0 / 2.0);
        z0 = z0 - f64::powf((x0 - z0) * (y0 - z0), 1.0 / 2.0);
        x0 = x;
        y0 = y;
    }
    x_vec.push(x0);
    y_vec.push(y0);
    vec![x_vec, y_vec]
}

pub fn print_res(num_array: Vec<i128>, depth: Option<f64>) -> i8 {
    let answer = count(num_array, depth);
    println!(
        "Ваш ответ\n\nАрефметическое: {:?}\nГеометрическое: {:?}",
        answer[0], answer[1]
    );
    return math_mods::exit_code_algos();
}
