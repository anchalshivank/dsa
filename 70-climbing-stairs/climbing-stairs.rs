impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n{
            1 | 2 => n,
            n => {

                    let mut dp = vec![0; n as usize];
                    dp[0] = 1;
                    dp[1] = 2;

                    for step in 2..(n as usize){
                        
                        dp[step] = dp[step-2]+ dp[step-1];

                    }

                    dp[n as usize -1]

            }
        }

        


    }
}