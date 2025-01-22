impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n == 0 {
            return 0;
        }

        let mut dp: Vec<i32> = vec![0; n];
        let mut max_len = 0;

        for i in 1..n {
            if chars[i] == ')' {
                if chars[i - 1] == '(' {
                    // Case when the previous character is '('
                    dp[i] = if i >= 2 { dp[i - 2] + 2 } else { 2 };
                } else if i > 1 && chars[i - 1] == ')' {
                    // Case when the previous character is ')' and there is a valid substring before it
                    let prev_len = dp[i - 1];
                    let open_idx = i as i32 - prev_len - 1;
                    if open_idx >= 0 && chars[open_idx as usize] == '(' {
                        dp[i] = dp[i - 1] + 2;
                        if open_idx as usize > 0 {
                            dp[i] += dp[open_idx as usize - 1];
                        }
                    }
                }
                max_len = max_len.max(dp[i]);
            }
        }

        max_len
    }
}
