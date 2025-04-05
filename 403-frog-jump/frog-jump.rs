use std::collections::HashMap;
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {

        //let's create a dp 
        //dp[pos][jump]
        //if the above is true it means that we can reach pos with this!! jump
        ///and this is not exactly the pos.. as if I directly do this position I will have memory limit exceed!!!!
        
        //dp[i][jump] means whatever the position is on ith idex.. for that jump if dp is true it means it is a suces

        let n = stones.len();
        //Now I do not not know the max size of the jump// But there it is mentione dthat we have a stones.length <=2000
        //let's see max possible jump
        //1 2 3 4 .... n = 2000
        //n*(n+1) = 4000
        //Jump shall not be more than 30

        let mut dp = vec![vec![false; n+1]; n];
        //to reach 0th index with 0 jump it is possible by default
        let mut map = HashMap::new();
        for (index, pos) in stones.iter().enumerate(){

            map.insert(pos, index);

        }
        dp[0][0] = true;



        for (index, &pos) in stones.iter().enumerate(){

            for jump in 1..n{

                if index == 0 {

                 if let Some(i)   = map.get(&1){

                    dp[1][1] = true;

                 } 

                }else{


                    //for current position does dp[i][jump] = true??
                    if dp[index][jump]{
                        //it means true
                        //Now we have three possibliye
                        let j1 = jump  -1;
                        let j2 = jump ;
                        let j3 = jump +1;

                        let n1 = pos+j1 as i32;
                        let n2 = pos+j2 as i32;
                        let n3 = pos+j3 as i32;

                        //we also need to get the index 
                        if let Some(&i) = map.get(&n1){
                            dp[i][j1] = true;
                        }
                        if let Some(&i) = map.get(&n2){
                            dp[i][j2] = true;
                        }
                        if let Some(&i) = map.get(&n3){
                            dp[i][j3] = true;
                        }

                    } 


                }

            }

        }

        //just iterat throug the last done!!
        for &val in &dp[n-1]{

            if val{
                return val;
            }

        }

        false

    }
}