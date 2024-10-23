use dialoguer::Select;
use std::collections::HashMap;

pub fn what_do() -> i8 {
    let items = vec!["Вернуться", "Повторить", "Завершить"];
    let mut action_code: HashMap<&str, i8> = HashMap::new();
    action_code.insert("Вернуться", 1);
    action_code.insert("Повторить", 0);
    action_code.insert("Завершить", -1);

    let selection = Select::new()
        .with_prompt("\n\nЧто делать дальше?")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();
    let ret = action_code.get(&items[selection]).unwrap();
    return *ret;
}
