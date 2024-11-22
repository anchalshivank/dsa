use std::collections::HashSet;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut left = 0usize;
        let mut right = 0usize;
        let mut set = HashSet::new();
        let mut max_sum: i64 = 0;
        let mut curr_sum: i64 = 0;

        while right < nums.len() {
            // Add element to the set and adjust the sum
            if !set.contains(&nums[right]) {
                set.insert(nums[right]);
                curr_sum += nums[right] as i64;
                right += 1;

                // Check if the set has reached size `k`
                if set.len() == k as usize {
                    // println!("{}", curr_sum);
                    max_sum = max_sum.max(curr_sum);
                    set.remove(&nums[left]);
                    curr_sum -= nums[left] as i64;
                    left += 1;
                }
            } else {
                // If duplicate, remove the leftmost element and adjust the sum
                set.remove(&nums[left]);
                curr_sum -= nums[left] as i64;
                left += 1;
            }
        }

        max_sum
    }
}
