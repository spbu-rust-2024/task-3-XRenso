use std::io;
/// Данная функция очищаешь терминал
/// С помощью большого отступа
pub fn clear_terminal() {
    println!("{}[2J", 27 as char);
}

/// Функция для получения поьзовательского ввода
/// Возвращает вектор с типом i128
///  # Принцип работы
/// Получаем на вход строку
/// ```
/// let mut nums = String::new();
///     println!("Введите числа:");
/// io::stdin()
///     .read_line(&mut nums)
///     .expect("Не удалось прочитать");
/// let user_input = nums.split(" ");
/// ```
/// Это будет разбитая строка через пробел.
/// Потом переводим этот вектор в тип `i128`
/// ```
/// let result = user_input
/// .map(|x| x.trim().parse::<i128>().expect("Не число"))
/// .collect::<Vec<i128>>();
/// ```
/// На выходе мы уже получаем `Vec<i128>`
/// # Пример использования
///
/// ```
/// result = small_logic::get_user_i128_input();
/// ```
pub fn get_user_i128_input() -> Vec<i128> {
    let mut nums = String::new();
    println!("Введите числа:");

    io::stdin()
        .read_line(&mut nums)
        .expect("Не удалось прочитать");
    let user_input = nums.split(" ");
    let result = user_input
        .map(|x| x.trim().parse::<i128>().expect("Не число"))
        .collect::<Vec<i128>>();
    return result;
}
