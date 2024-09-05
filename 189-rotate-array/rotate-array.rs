impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {

        let mut k =k as usize %nums.len();
        let len = nums.len();

        nums[len-k..].reverse();
        nums[0..len-k].reverse();
        nums.reverse();

        
    }
}