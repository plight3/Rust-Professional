use rand::Rng;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

fn pollards_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    if n % 3 == 0 {
        return 3;
    }
    if n % 5 == 0 {
        return 5;
    }

    let mut rng = rand::rng();
    let mut x = 2;
    let mut y = 2;
    let mut c = rng.random_range(1..n);
    let mut d = 1;

    while d == 1 {
        x = (mod_pow(x, 2, n) + c) % n;
        y = (mod_pow(y, 2, n) + c) % n;
        y = (mod_pow(y, 2, n) + c) % n;
        d = gcd((x as i128 - y as i128).abs() as u128, n);
    }

    if d == n {
        pollards_rho(n)
    } else {
        d
    }
}

pub fn find_max_prime_factor(num: u128) -> u128 {
    let mut largest_factor = 1;
    let mut n = num;
    // 处理小质因数
    for &p in &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43] {
        while n % p == 0 {
            largest_factor = p;
            n /= p;
        }
    }

    // 使用 Pollard's Rho 分解大质因数
    while n > 1 {
        if is_prime(n) {
            largest_factor = n;
            break;
        }
        let factor = pollards_rho(n);
        largest_factor = largest_factor.max(factor);
        n /= factor;
    }

    largest_factor
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}