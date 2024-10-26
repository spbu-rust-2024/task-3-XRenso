use std::collections::HashMap;

use crate::{math_mods, small_logic};

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

pub fn count(num_array: Vec<i128>) -> i8 {
    let result: Vec<i128>;
    if num_array.is_empty() {
        result = small_logic::get_user_i128_input();
    } else {
        result = num_array;
    }
    let mut counter_elements: HashMap<i128, i128> = HashMap::new();
    for i in result.iter() {
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
    println!("Ваш ответ - {:?}", get_max_values(counter_elements));

    return math_mods::exit_code_algos();
}
