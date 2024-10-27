use dialoguer::Select;
use std::collections::HashMap;
/// Функция для выбора дальнейших действий пользователем
///
/// # Принцип работы
/// Создается список из элементов для выбора при помощи контейнера
/// `dialoguer`
/// ```
/// let items = vec!["Вернуться", "Повторить", "Завершить"];
/// let selection = Select::new()
///     .with_prompt("\n\nЧто делать дальше?")
///     .default(0)
///     .items(&items)
///     .interact()
///     .unwrap();
/// ```
/// Выбранный ответ мы сопоставляем при помощи `Hashmap`
/// и получаем код возврата
/// ```
/// let mut action_code: HashMap<&str, i8> = HashMap::new();
/// action_code.insert("Вернуться", 1);
/// action_code.insert("Повторить", 0);
/// action_code.insert("Завершить", -1);
/// let ret = action_code.get(&items[selection]).unwrap();
/// ```
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
