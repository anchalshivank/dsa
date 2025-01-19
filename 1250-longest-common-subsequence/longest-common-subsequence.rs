impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {


        let t1 : Vec<char>= text1.chars().collect();
        let t2 : Vec<char>= text2.chars().collect();
        // in simple terms lcs[0, 0] = 
        //if equal then 1+lcs[1,1]
        //else
        // max(lcs[0,1] , lcs[1,0])
        //but we know the value of last n 
        //so we will do from bottom 
        let len1 = t1.len();
        let len2 = t2.len();

        let mut lcs = vec![vec![0; len2+1]; len1+1];
    
        for (i, ch1 ) in t1.iter().enumerate(){
            for (j, ch2) in t2.iter().enumerate(){

                if ch1 == ch2 {
                    //it means it is equal 
                    //the lcs shall be
                    lcs[i+1][j+1] = lcs[i][j] + 1;
                }else{

                    //we will just take the max of any 
                    lcs[i+1][j+1] = lcs[i+1][j].max(lcs[i][j+1]);

                }

            }
        }        
        lcs[len1][len2]
    }
}