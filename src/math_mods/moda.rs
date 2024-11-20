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
    let mut max_value: i128 = 0;

    for (k, v) in val_counts.iter() {
        if max_value == *v {
            maxed.push(*k);
        } else if max_value < *v {
            maxed.clear();
            max_value = *v;
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
}

pub fn print_res(num_array: Vec<i128>) -> i8 {
    let counter_elements = count(num_array);
    println!("Ваш ответ - {:?}", get_max_values(counter_elements));
    return math_mods::exit_code_algos();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn vector_elements_same_any_order(vec_from: Vec<i128>, vec_in: Vec<i128>) -> bool {
        vec_from.iter().all(|x| vec_in.contains(&x))
    }

    #[test]
    fn moda_equal_count_numbers() {
        let input: Vec<i128> = vec![1, 2, 3, 4];
        let expected_answer = vec![1, 2, 3, 4];
        let result = vector_elements_same_any_order(expected_answer, get_max_values(count(input)));
        assert_eq!(result, true)
    }

    #[test]
    fn moda_clearly_defined() {
        let input: Vec<i128> = vec![4, 4, 4, 4, 1, 2, 3, 4, 4];
        let expected_answer = vec![4];
        let program_answer = get_max_values(count(input));
        let result = vector_elements_same_any_order(expected_answer, program_answer);

        assert_eq!(result, true)
    }

    #[test]
    fn moda_few_max_counts() {
        let input: Vec<i128> = vec![4, 4, 4, 3, 3, 3, 1, 2, 5, 6, 7];
        let expected_answer = vec![4, 3];
        let program_answer = get_max_values(count(input));
        let result = vector_elements_same_any_order(expected_answer, program_answer);

        assert_eq!(result, true)
    }
}
