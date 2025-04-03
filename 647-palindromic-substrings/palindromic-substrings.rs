impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![false; n + 1]; n + 1]; // Extra padding
        let mut count = 0;

        for gap in 0..n {
            let mut row = 1; // Start from 1 to align with the padded matrix
            let mut col = gap + 1;
            while col <= n {
                if row == col {
                    dp[row][col] = true;
                } else if col - row == 1 {
                    dp[row][col] = chars[row - 1] == chars[col - 1];
                } else {
                    dp[row][col] = chars[row - 1] == chars[col - 1] && dp[row + 1][col - 1];
                }

                if dp[row][col] {
                    count += 1;
                }

                row += 1;
                col += 1;
            }
        }

        count
    }
}
