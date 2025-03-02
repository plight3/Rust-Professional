fn check_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    } else if num <= 3 {
        return true;
    } else if num % 2 == 0 || num % 3 == 0 {
        return false;
    } else {
        let mut i = 5;
        while i * i <= num {
            if num % i == 0 || num % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        return true;
    }
}


pub fn goldbach_conjecture() -> String {
    let mut odd_composite = 9;
    let mut vec_ret = Vec::new();
    while vec_ret.len() != 2 {
        if odd_composite % 2 !=0 && !check_prime(odd_composite) {
            for k in 1.. {
                if odd_composite <= 2*k*k {
                    vec_ret.push(odd_composite);
                    break;
                }
                if check_prime(odd_composite - 2*k*k) {
                    break;
                }
            }
        }
        odd_composite += 2
    }
    let str_ret = format!("{},{}", vec_ret[0], vec_ret[1]);
    return str_ret.to_string();
}