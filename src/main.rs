use dialoguer::Select;
use std::collections::HashMap;
mod constants;
mod handler_task;
mod math_mods;
mod small_logic;

fn main() {
    small_logic::clear_terminal();
    let items = vec![
        "Среднее арифметическое",
        "Среднее геометрическое",
        "Среднее степенное",
        "Среднее арефметико-геометрическое",
        "Модифицированное арифметико-геометрическое среднее",
        "Среднее по Колмогорову",
        "Среднее усеченное",
        "Винсоризованное среднее",
        "Медиана",
        "Мода",
    ];

    let mut action: HashMap<&str, &str> = HashMap::new();
    action.insert("Среднее арифметическое", "arth_mean");
    action.insert("Среднее геометрическое", "geo_mean");
    action.insert("Среднее степенное", "deg_mean");
    action.insert("Среднее арефметико-геометрическое", "arth_geo_mean");
    action.insert(
        "Модифицированное арифметико-геометрическое среднее",
        "modif_arth_geo_mean",
    );
    action.insert("Среднее по Колмогорову", "kolmogor_mean");
    action.insert("Среднее усеченное", "trim_mean");
    action.insert("Винсоризованное среднее", "vinzor_mean");
    action.insert("Медиана", "median");
    action.insert("Мода", "moda");

    println!("{}", constants::PROG_NAME);

    let selection = Select::new()
        .with_prompt("Что будем делать?")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();

    let func_code = action.get(&items[selection]).unwrap();
    let res = handler_task::get_command(func_code);
    if res == 1 {
        main();
    }
}
