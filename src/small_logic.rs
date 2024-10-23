use std::io;
pub fn clear_terminal() {
    println!("{}[2J", 27 as char);
}

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
