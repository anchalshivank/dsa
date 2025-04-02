impl Solution {
    pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {

        fn helper(
            i: usize,
            nums: &Vec<i32>
        ) -> i32 {


            if i >= nums.len(){
                return 0;
            }

            //let 
            let curr = nums[i];
            let mut curr_sum = nums[i];
            let mut index = i+1;

            while index < nums.len() && nums[index] == curr{

                curr_sum +=nums[i];
                index+=1;

            }

            while index < nums.len() && nums[index] == curr+1{

                index+=1;

            }

            //Now we have two choices to include this sum or not!!!!!
            let a = curr_sum+ helper(index, nums);
            let b = helper(i+1, nums);

            a.max(b)


        }

        let n = nums.len();
        let mut dp = vec![0; n+1];



        nums.sort();

        for i in (0..=n).rev(){

            if i==n {
                dp[i] = 0;
            }else{

                let curr = nums[i];
                let mut curr_sum = nums[i];
                let mut index = i+1;

                while index < n && nums[index] == curr{
                    curr_sum+=curr;
                    index+=1;
                }

                //ignore just curr+1

                while index < n && nums[index] == curr+1{
                    index+=1;
                }
            
                dp[i] = i32::max(curr_sum + dp[index], dp[i+1]);

            }


            

        }

        dp[0]
        
    }
}