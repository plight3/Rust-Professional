pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1.0;
    }

    let mut probability = 1.0;
    for i in 0..n {
        probability *= (365 - i) as f64 / 365.0;
    }
    
    let str_float_round = format!("{:.4}", 1.0 - probability);
    let parsed_num: f64 = str_float_round.parse().unwrap();
    return parsed_num;
}
