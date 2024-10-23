use crate::{math_mods, small_logic};
use std::process::exit;

fn unknown_cmd() -> i8 {
    println!("Пока так не умею");
    return 1;
}

pub fn get_command(func_code: &str) -> i8 {
    let mut res: i8;
    if func_code == "arth_mean" {
        res = math_mods::arithmetic_mean();
    } else if func_code == "geo_mean" {
        res = math_mods::geometry_mean();
    } else if func_code == "deg_mean" {
        res = math_mods::degree_mean();
    } else if func_code == "arth_geo_mean" {
        res = math_mods::arth_geo_mean();
    } else if func_code == "modif_arth_geo_mean" {
        res = math_mods::modif_arth_geo_mean();
    } else {
        res = unknown_cmd();
    }
    if res == 0 {
        small_logic::clear_terminal();
        res = get_command(func_code);
    } else if res == -1 {
        println!("Спасибо за использование продукта MEGA MATH!!");
        exit(0);
    }
    return res;
}
