impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        
        if s.is_empty(){
            return 0;
        }

        let chars: Vec<char>  = s.chars().collect();

        let n = chars.len();
        let mut dp = vec![0; n+1];

        dp[0] = 1;
        dp[1] = if chars[0] != '0' {1} else {0};

        for i in 2..=n{
            
            let one_digit = chars[i-1].to_digit(10).unwrap_or(0);
            let two_digit = chars[i-2].to_digit(10).unwrap_or(0) * 10 + one_digit;
            if one_digit!=0{

                dp[i] +=dp[i-1];
            }
            // println!("{} {} ", one_digit, two_digit);
            //we need to confirm for two digit

            if (10..=26).contains(&two_digit){
                dp[i] +=dp[i-2];
            }
            // println!("{:?}", dp);

        }

        // println!("{:?}", dp);
        dp[n]



    }
}