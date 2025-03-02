pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut number = number;
    let mut u128_ret = 1;
    while number % 2 == 0 {
        u128_ret = 2;
        number /= 2;
    }
    while number % 3 == 0 {
        u128_ret = 3;
        number /= 3;
    }
    let mut i = 5;
    while i * i <= number {
        if number % i == 0 {
            u128_ret = i;
            number /= i;
        } else if number % (i + 2) == 0 {
            u128_ret = i + 2;
            number /= (i + 2);
        }
        i += 6;
    }
    if number > 1 {
        u128_ret = number;
    }
    return number;
}