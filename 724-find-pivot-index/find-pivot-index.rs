impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        
        let mut left_sum = 0;
        let total_sum: i32 = nums.iter().sum();

        for (index, num) in nums.iter().enumerate(){

            if left_sum == total_sum - left_sum - num{
                return index as i32;

            }else{
                left_sum+=num;
            }
            

        }

        -1


    }
}