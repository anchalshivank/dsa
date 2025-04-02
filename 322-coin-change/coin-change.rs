impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let n = amount as usize;
        let mut dp = vec![i32::MAX; n + 1];

        dp[0] = 0; // Base case: amount 0 needs 0 coins

        for &coin in &coins {
            for i in coin as usize..=n {
                if dp[i - coin as usize] != i32::MAX {
                    dp[i] = dp[i].min(1 + dp[i - coin as usize]);
                }
            }
        }

        if dp[n] == i32::MAX {
            -1
        } else {
            dp[n]
        }
    }
}
