pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    let parts: Vec<&str> = num_str.split('(').collect();
    let num_str = parts[0];
    let from_base: u32 = parts[1].trim_end_matches(')').parse().unwrap();

    let mut decimal_num = u32::from_str_radix(num_str, from_base).unwrap();
    let mut vec_trans_num = String::new();
    let digits = "0123456789abcdefghijklmnopqrstuvwxyz";
    while decimal_num > 0 {
        let remainder = decimal_num % to_base;
        decimal_num /= to_base;
        vec_trans_num.push(digits.chars().nth(remainder as usize).unwrap());
    }

    // 因为计算是从低位到高位，所以需要反转字符串
    vec_trans_num.chars().rev().collect()
}