use crate::{math_mods, small_logic};
use std::io;

/// Функция для расчета арефметико-геометрического среднего
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов.
/// Должен состоять лишь из двух чисел
/// * `depth` - число с плавающей точкой типа `f64`,
/// которое мы используем для глубины расчетов. Получается при помощи флага
/// из консоли
/// # Принцип работы
/// Мы создаем 2 вектора, которые будут отвечать за последовательности a и b
/// ```
/// let mut a = result[0] as f64;
/// let mut b = result[1] as f64;
///```
/// P.S `result` это пользовательский ввод.
///
/// Пробегаемся по циклу, что равен глубине расчетов. Таким образом мы
/// заполняем наши массивы и получаем искомую последовательность.
/// ```
/// let mut a0 = a;
/// let mut b0 = b;
/// for _ in 0..deep {
///     a_vec.push(a0);
///     b_vec.push(b0);
///     a = (a0 + b0) / 2.0;
///     b = f64::powf(a0 * b0, 1.0 / 2.0);
///     a0 = a;
///     b0 = b;
/// }
/// a_vec.push(a0);
/// b_vec.push(b0);
/// ```
///
/// # Результат
/// По итогу работы программы мы получаем 2 вектора типа `f64` и
/// выводим их
/// ```
///
/// let mut a_vec: Vec<f64> = Vec::new();
/// let mut b_vec: Vec<f64> = Vec::new();
/// println!(
/// "Ваш ответ\n\nАрефметическое: {:?}\nГеометрическое: {:?}",
/// a_vec, b_vec
/// );
/// ```

pub fn print_res(num_array: Vec<i128>, depth: Option<f64>) -> i8 {
    let mut user_input = String::new();
    let deep: i128;
    let result: Vec<i128>;
    if depth == None {
        println!("Введите глубину расчета: ");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Не удалось прочитать");
        deep = user_input
            .trim()
            .parse()
            .expect("Неверный тип входных данных");
    } else {
        deep = depth.unwrap() as i128;
    }
    if deep < 1 {
        panic!("Невалидные входные данные: Глубина расчетов меньше 1");
    }
    if num_array.is_empty() {
        result = small_logic::get_user_i128_input();
    } else {
        result = num_array;
    }
    if result.len() != 2 {
        panic!("Невалидные входные данные: должно быть 2 числа");
    }

    let mut a = result[0] as f64;
    let mut b = result[1] as f64;
    let mut a0 = a;
    let mut b0 = b;

    let mut a_vec: Vec<f64> = Vec::new();
    let mut b_vec: Vec<f64> = Vec::new();

    for _ in 0..deep {
        a_vec.push(a0);
        b_vec.push(b0);
        a = (a0 + b0) / 2.0;
        b = f64::powf(a0 * b0, 1.0 / 2.0);
        a0 = a;
        b0 = b;
    }
    a_vec.push(a0);
    b_vec.push(b0);

    println!(
        "Ваш ответ\n\nАрефметическое: {:?}\nГеометрическое: {:?}",
        a_vec, b_vec
    );
    return math_mods::exit_code_algos();
}
