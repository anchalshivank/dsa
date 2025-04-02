impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        
        let n = matrix.len();
        let m = matrix[0].len();

        let mut dp = vec![vec![0;m+1]; n+1];

        let mut a = i32::MIN;

        for i in 1..=n{
            for j in 1..=m{

                //max square that can be formed of any length is a 
                //Now lets compare 
                if matrix[i-1][j-1] == '1'{
                    dp[i][j] = i32::min(i32::min(dp[i-1][j], dp[i][j-1]), dp[i-1][j-1])+1;
                    a = a.max(dp[i][j]);
                }
            }
        }
        a*a
    }
}