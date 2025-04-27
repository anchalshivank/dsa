impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = triangle.clone(); // start from same size

        // start from second last row and move upwards
        for i in (0..n-1).rev() {
            for j in 0..dp[i].len() {
                dp[i][j] += dp[i+1][j].min(dp[i+1][j+1]);
            }
        }

        dp[0][0]
    }
}
