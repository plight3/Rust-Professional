fn main() {
    let input = "2025-05-01";
    let (year, month, day) = parse_date(input);
    let day_of_year = day_of_year_2025(month, day);
    let week_num = ((day_of_year - 1) / 7 % 52) + 1; // 修正周数计算
    let weekday = (3 + day_of_year - 1) % 7;
    let remaining_days = 365 - day_of_year;
    // 修正春节天数计算
    let days_to_spring = if day_of_year <= 29 {
        29 - day_of_year
    } else {
        let days_left_2025 = 365 - day_of_year;
        let days_in_2026 = 31 + 17; // 1月31天 + 2月17天
        days_left_2025 + days_in_2026
    };
    let (next_year, next_month, next_day) = next_trading_day(year, month, day);
    let current_total = total_days_2025_base(year, month, day);
    let next_total = total_days_2025_base(next_year, next_month, next_day);
    let days_to_next_trading = next_total - current_total - 1;

    println!(
        "{},{},{},{},{},{}",
        week_num, weekday, day_of_year, remaining_days, days_to_spring, days_to_next_trading
    );
}

fn parse_date(s: &str) -> (i32, u32, u32) {
    let parts: Vec<&str> = s.split('-').collect();
    let year = parts[0].parse().unwrap();
    let month = parts[1].parse().unwrap();
    let day = parts[2].parse().unwrap();
    (year, month, day)
}

fn day_of_year_2025(month: u32, day: u32) -> u32 {
    let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days = day;
    for m in 0..(month - 1) as usize {
        days += months[m];
    }
    days
}

fn next_day(year: i32, month: u32, day: u32) -> (i32, u32, u32) {
    let months_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let (mut new_year, mut new_month, mut new_day) = (year, month, day + 1);

    if new_day > months_days[(new_month - 1) as usize] {
        new_day = 1;
        new_month += 1;
        if new_month > 12 {
            new_month = 1;
            new_year += 1;
        }
    }
    (new_year, new_month, new_day)
}

fn is_trading_day(year: i32, month: u32, day: u32) -> bool {
    let is_holiday = 
        (month == 1 && day == 1) ||                        // 元旦
        (year == 2025 && ( (month == 1 && day >= 28) ||     // 春节1.28-2.3
                            (month == 2 && day <= 3) )) ||
        (month == 5 && day <= 3) ||                        // 劳动节5.1-5.3
        (month == 10 && day == 1);                         // 国庆
    
    if is_holiday { return false; }
    
    let weekday = compute_weekday(year, month, day);
    !(weekday == 0 || weekday == 6)  // 排除周末
}

fn compute_weekday(year: i32, month: u32, day: u32) -> u32 {
    let total = total_days_2025_base(year, month, day);
    (total as u32 + 2) % 7
}

fn total_days_2025_base(year: i32, month: u32, day: u32) -> i32 {
    let mut total = 0;
    for y in 2025..year {
        total += if is_leap_year(y) { 366 } else { 365 };
    }
    total += day_of_year_in_year(year, month, day) as i32;
    total
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn day_of_year_in_year(year: i32, month: u32, day: u32) -> u32 {
    let months_days = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut days = day;
    for m in 0..(month - 1) as usize {
        days += months_days[m];
    }
    days
}

fn next_trading_day(start_year: i32, start_month: u32, start_day: u32) -> (i32, u32, u32) {
    let (mut y, mut m, mut d) = (start_year, start_month, start_day);
    loop {
        (y, m, d) = next_day(y, m, d);
        if is_trading_day(y, m, d) {
            return (y, m, d);
        }
    }
}