impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let  mut result = Vec::new();
        result.push(nums[0]);
        let mut i = 0;
        for &val in nums[1..].iter(){

            result.push(result[i]+val);
            i+=1;

        }

        result
    }
}