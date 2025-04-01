use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        
        //Ok let do something!!!

        //one thing that comes to mind is that!!
        // if s[a..b] is matching with any string than s[b] = s[0..a]
        //let's make a set 

        let set : HashSet<String> = word_dict.into_iter().collect();

        // println!("{:?}", set);
        let n = s.len();
        let mut dp = vec![false; n+1];
        dp[0] = true;
        let mut i = 0;

        for i in 1..=n{
            for j in 0..i{
                if dp[j] && set.contains(&s[j..i]){
                    dp[i] = true;
                    break;
                }
            }
        }

        // println!("{:?}", dp);
        dp[n]

    }
}