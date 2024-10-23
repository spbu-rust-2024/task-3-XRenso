use dialoguer::Select;
use std::collections::HashMap;
mod handler_task;
mod math_mods;
mod small_logic;
const PROG_NAME: &str = "


██████   ██████ ██████████   █████████    █████████    
░░██████ ██████ ░░███░░░░░█  ███░░░░░███  ███░░░░░███   
 ░███░█████░███  ░███  █ ░  ███     ░░░  ░███    ░███   
 ░███░░███ ░███  ░██████   ░███          ░███████████   
 ░███ ░░░  ░███  ░███░░█   ░███    █████ ░███░░░░░███   
 ░███      ░███  ░███ ░   █░░███  ░░███  ░███    ░███   
 █████     █████ ██████████ ░░█████████  █████   █████  
░░░░░     ░░░░░ ░░░░░░░░░░   ░░░░░░░░░  ░░░░░   ░░░░░   
                                                        
                                                        
                                                        
 ██████   ██████   █████████   ███████████ █████   █████
░░██████ ██████   ███░░░░░███ ░█░░░███░░░█░░███   ░░███ 
 ░███░█████░███  ░███    ░███ ░   ░███  ░  ░███    ░███ 
 ░███░░███ ░███  ░███████████     ░███     ░███████████ 
 ░███ ░░░  ░███  ░███░░░░░███     ░███     ░███░░░░░███ 
 ░███      ░███  ░███    ░███     ░███     ░███    ░███ 
 █████     █████ █████   █████    █████    █████   █████
░░░░░     ░░░░░ ░░░░░   ░░░░░    ░░░░░    ░░░░░   ░░░░░ 


";

fn main() {
    small_logic::clear_terminal();
    let items = vec![
        "Среднее арифметическое",
        "Среднее геометрическое",
        "Среднее степенное",
    ];
    let mut action: HashMap<&str, &str> = HashMap::new();
    action.insert("Среднее арифметическое", "arth_mean");
    action.insert("Среднее геометрическое", "geo_mean");
    action.insert("Среднее степенное", "deg_mean");

    println!("{}", PROG_NAME);

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
