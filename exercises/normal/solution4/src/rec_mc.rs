pub fn dp_rec_mc(amount: u32) -> u32 {
    let vec_coins = vec![1, 2, 5, 10, 20, 30, 50, 100];
    let mut vec_dp = vec![u32::MAX; (amount+1) as usize];
    vec_dp[0] = 0;
    let amount = amount as usize;
    for i in 1..=amount as usize{
        let mut min_count = vec_dp[i];
        for &coin in vec_coins.iter() {
            if i >= coin && vec_dp[i-coin]+1 < min_count {
                min_count = vec_dp[i-coin]+1;
            } 
        }
        vec_dp[i] = min_count;
    }
    return vec_dp[amount];
}
