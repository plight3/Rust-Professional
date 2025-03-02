pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let threshold: usize = threshold as usize;
    let mut vec_fib = vec![0;threshold+1];
    vec_fib[0] = 0;
    let mut sum = 1;
    if threshold > 1 {
        vec_fib[1] = 1;
        for i in 2..=threshold {
            vec_fib[i] = vec_fib[i-1] + vec_fib[i-2];
            if vec_fib[i] >= threshold {
                break;
            }
            if vec_fib[i] & 1 != 0 {
                sum += vec_fib[i];
            }
        }
    } else {
        sum = 0;
    }
    return sum as u32;
}
