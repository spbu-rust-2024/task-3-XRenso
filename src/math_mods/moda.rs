use std::collections::HashMap;

use crate::{math_mods, small_logic};

/// Функция для расчета самого частого числа из Hashmap
/// # Входные данные
/// * `val_counts` - `Hashmap` из целочисленных тип `HashMap<i128,i128>`,
/// который мы получаем после обработки пользовательского ввода
///
/// # Принцип работы
/// Мы пробегаемся по всей `HashMap` и записываем числа с максимальным значением
/// ```
/// let mut maxed: Vec<i128> = Vec::new();
/// let mut idx = 0;
/// let mut max_value: i128 = 0;
/// for (k, v) in val_counts.iter() {
///     if idx == 0 {
///         max_value = *v;
///     }
///     idx += 1;
///     if max_value == *v {
///         maxed.push(*k);
///     }
/// }
/// ```
fn get_max_values(val_counts: HashMap<i128, i128>) -> Vec<i128> {
    let mut maxed: Vec<i128> = Vec::new();
    let mut idx = 0;
    let mut max_value: i128 = 0;
    for (k, v) in val_counts.iter() {
        if idx == 0 {
            max_value = *v;
        }
        idx += 1;
        if max_value == *v {
            maxed.push(*k);
        }
    }
    maxed
}

/// Функция для расчета моды
/// # Входные данные
/// * `num_array` - вектор из целочисленных типа `i128`,
/// которые мы получаем при передаче, если мы запускаем при помощи флагов
///
/// # Принцип работы
/// Мы создаем `HashMap<i128,i128>`, в которую записываем число и сколько раз оно встречается и передаем отстортированный вид в `get_max_values`
/// ```
/// let mut counter_elements: HashMap<i128, i128> = HashMap::new();
///     for i in result_user_input.iter() {
///         if counter_elements.contains_key(i) {
///             *counter_elements.get_mut(i).unwrap() += 1;
///         } else {
///             counter_elements.insert(*i, 1);
///         }
///     }
/// counter_elements
///         .iter()
///         .min_by(|a, b| a.1.cmp(&b.1))
///         .map(|(k, _v)| k);
/// ```

pub fn count(num_array: Vec<i128>) -> HashMap<i128, i128> {
    let result_user_input: Vec<i128>;
    if num_array.is_empty() {
        result_user_input = small_logic::get_user_i128_input();
    } else {
        result_user_input = num_array;
    }
    let mut counter_elements: HashMap<i128, i128> = HashMap::new();
    for i in result_user_input.iter() {
        if counter_elements.contains_key(i) {
            *counter_elements.get_mut(i).unwrap() += 1;
        } else {
            counter_elements.insert(*i, 1);
        }
    }
    counter_elements
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k);
    counter_elements
}

pub fn print_res(num_array: Vec<i128>) -> i8 {
    let counter_elements = count(num_array);
    println!("Ваш ответ - {:?}", get_max_values(counter_elements));
    return math_mods::exit_code_algos();
}
