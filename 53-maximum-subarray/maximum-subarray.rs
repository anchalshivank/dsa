impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {

        let mut max_sum = i32::MIN;
        let mut curr = 0;

        for i in 0..nums.len(){

            curr+=nums[i];
            max_sum = max_sum.max(curr);
            if curr<0{
                curr = 0;

            }

        }
        max_sum

        
    }
}