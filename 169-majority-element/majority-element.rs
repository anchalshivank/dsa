impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut mj = nums[0];
        let mut count = 1;
        for i in 1..nums.len(){
            let num = nums[i];
            if count == 0 {
                mj = num;
            }
            if mj == num{
                count+=1;
            }else{
                count -=1;
            }

            
        }
    
        mj

    }
}