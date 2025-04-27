impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let n = s.len();
        let m = t.len();
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        let mut dp = vec![0; m + 1];
        dp[0] = 1; // Empty t matches empty s

        for j in 0..n {
            for i in (0..m).rev() {
                if s_chars[j] == t_chars[i] {
                    dp[i + 1] += dp[i];
                }
            }
        }

        dp[m]
    }
}
