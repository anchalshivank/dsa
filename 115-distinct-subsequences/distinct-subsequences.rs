impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {

        let n = s.len();
        let m = t.len();
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let mut dp = vec![vec![0; n+1]; m+1 ];
        dp[0].fill(1);

        // println!("{:?}", dp);
        for i in 0..m{

            for j in 0..n{

                if s_chars[j] == t_chars[i]{

                    dp[i+1][j+1] = dp[i][j]+ dp[i+1][j];

                }else{
                    dp[i+1][j+1] = dp[i+1][j];
                }

            }

        }

        // for row in dp.iter(){

        //     println!("{:?}", row);

        // }


        dp[m][n]

        
    }
}