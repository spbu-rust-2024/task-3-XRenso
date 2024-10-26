use crate::{math_mods, small_logic};
use std::process::exit;

fn unknown_cmd() -> i8 {
    println!("Пока так не умею");
    return 1;
}

pub fn get_command(func_code: &str) -> i8 {
    let mut res = match func_code {
        "arth_mean" => math_mods::arithmetic_mean(),
        "geo_mean" => math_mods::geometry_mean(),
        "deg_mean" => math_mods::degree_mean(),
        "arth_geo_mean" => math_mods::arth_geo_mean(),
        "modif_arth_geo_mean" => math_mods::modif_arth_geo_mean(),
        "kolmogor_mean" => math_mods::kolmogor_mean(),
        "trim_mean" => math_mods::trim_mean(),
        "vinzor_mean" => math_mods::vinzor_mean(),
        "median" => math_mods::median(),
        "moda" => math_mods::moda(),
        "mean_deviation" => math_mods::deaviation_mean(),
        _ => unknown_cmd(),
    };

    if res == 0 {
        small_logic::clear_terminal();
        res = get_command(func_code);
    } else if res == -1 {
        println!("Спасибо за использование продукта MEGA MATH!!");
        exit(0);
    }
    return res;
}
