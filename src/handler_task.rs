use crate::{math_mods, small_logic};
use std::process::exit;

fn unknown_cmd() -> i8 {
    panic!("Неправильный код команды");
}
/// Функция для обработки пользовательского вода и запуска функции на его основе
/// # Входные данные
/// * `func_code` - тип `&str`, получаем код функции, которую хотим запустить
/// * `num_array` - типо `Vec<i128>`, получаем пользовательский ввод из
/// терминала
/// * `depth` - тип `Option<f64>`, необязательный параметр, который нужен
/// некоторым функциям для разных уточнений работы
///
/// # Прицип работы
/// Получив код функции мы обрабатываем его при помощи `match`, запуская нужную
/// функцию из модуля `math_mods`
/// ```
/// let mut res = match func_code {
///     "arth_mean" => math_mods::arithmetic_mean(num_array.clone()),
///     "geo_mean" => math_mods::geometry_mean(num_array.clone()),
///     "deg_mean" => math_mods::degree_mean(num_array.clone(), depth),
///     "arth_geo_mean" => math_mods::arth_geo_mean(num_array.clone(), depth),
///     "modif_arth_geo_mean" => math_mods::modif_arth_geo_mean(num_array.clone(), depth),
///     "kolmogor_mean" => math_mods::kolmogor_mean(num_array.clone()),
///     "trim_mean" => math_mods::trim_mean(num_array.clone(), depth),
///     "vinzor_mean" => math_mods::vinzor_mean(num_array.clone(), depth),
///     "median" => math_mods::median(num_array.clone()),
///     "moda" => math_mods::moda(num_array.clone()),
///     "mean_deviation" => math_mods::deaviation_mean(num_array.clone()),
///     "mean_square_dev" => math_mods::mean_square_dev(num_array.clone()),
///     "linear_coeff_deviation" => math_mods::linear_coeff_deviation(num_array.clone()),
///     "square_coeff_deviation" => math_mods::square_coeff_deviation(num_array.clone()),
///     "dispersion" => math_mods::dispersion(num_array.clone()),
///     _ => unknown_cmd(),
/// };
/// ```
///
/// На основе кода возврата функции, мы запускаем снова функцию, либо выходим,
/// либо возвращаемся к главному меню
/// ```
/// if res == 0 {
///     small_logic::clear_terminal();
///     res = get_command(func_code, num_array, depth);
/// } else if res == -1 {
///     println!("Спасибо за использование продукта MEGA MATH!!");
///     exit(0);
/// }
/// return res;
/// ```
pub fn get_command(func_code: &str, num_array: Vec<i128>, depth: Option<f64>) -> i8 {
    let mut res = match func_code {
        "arth_mean" => math_mods::arithmetic_mean(num_array.clone()),
        "geo_mean" => math_mods::geometry_mean(num_array.clone()),
        "deg_mean" => math_mods::degree_mean(num_array.clone(), depth),
        "arth_geo_mean" => math_mods::arth_geo_mean(num_array.clone(), depth),
        "modif_arth_geo_mean" => math_mods::modif_arth_geo_mean(num_array.clone(), depth),
        "kolmogor_mean" => math_mods::kolmogor_mean(num_array.clone()),
        "trim_mean" => math_mods::trim_mean(num_array.clone(), depth),
        "vinzor_mean" => math_mods::vinzor_mean(num_array.clone(), depth),
        "median" => math_mods::median(num_array.clone()),
        "moda" => math_mods::moda(num_array.clone()),
        "mean_deviation" => math_mods::deaviation_mean(num_array.clone()),
        "mean_square_dev" => math_mods::mean_square_dev(num_array.clone()),
        "linear_coeff_deviation" => math_mods::linear_coeff_deviation(num_array.clone()),
        "square_coeff_deviation" => math_mods::square_coeff_deviation(num_array.clone()),
        "dispersion" => math_mods::dispersion(num_array.clone()),
        _ => unknown_cmd(),
    };

    if res == 0 {
        small_logic::clear_terminal();
        res = get_command(func_code, num_array, depth);
    } else if res == -1 {
        println!("Спасибо за использование продукта MEGA MATH!!");
        exit(0);
    }
    return res;
}
