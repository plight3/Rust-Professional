pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析出生日期
    let parsed_data: Vec<&str> = time.split("-").collect();
    let birth_year: i32 = parsed_data[0].parse().unwrap();
    let birth_month: i32 = parsed_data[1].parse().unwrap();

    // 根据人员类型设置退休参数
    let (base_retirement_age, delay_rate, max_delay) = match tp {
        "男职工" => (60, 1.0 / 4.0, 36),
        "原法定退休年龄55周岁女职工" => (55, 1.0 / 4.0, 36),
        "原法定退休年龄50周岁女职工" => (50, 1.0 / 2.0, 60),
        _ => panic!("未知的员工类型"),
    };

    // 计算原始退休年份和月份
    let base_retirement_year = birth_year + base_retirement_age;
    let base_retirement_month = birth_month;

    // 延迟退休的起始年份
    let start_delay_year = 2025;
    let start_delay_month = 0;

    // 计算延迟月数
    let mut delay_months = 0;
    if base_retirement_year >= start_delay_year
        || (base_retirement_year == start_delay_year && base_retirement_month >= start_delay_month)
    {
        let months_since_start =
            (base_retirement_year - start_delay_year) * 12 + (base_retirement_month - start_delay_month);
        delay_months = (months_since_start as f64 * delay_rate).ceil() as i32;
        delay_months = delay_months.min(max_delay);
    }

    // 计算实际退休日期
    let total_months = base_retirement_year * 12 + base_retirement_month + delay_months;
    let mut retirement_year = total_months / 12;
    let mut retirement_month = total_months % 12;
    if retirement_month == 0 {
        retirement_month = 12;
        retirement_year -= 1;
    }

    // 计算实际退休年龄
    let retirement_age = base_retirement_age as f64 + delay_months as f64 / 12.0;

    // 格式化退休年龄
    let retirement_age_str = if retirement_age.fract() == 0.0 {
        format!("{:.0}", retirement_age)
    } else {
        format!("{:.2}", retirement_age)
    };
    // 返回结果
    let string_ret = format!(
        "{:}-{:02},{},{}",
        retirement_year, retirement_month, retirement_age_str, delay_months
    );
    println!("{}", string_ret);
    return string_ret;
}