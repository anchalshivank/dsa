impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        fn helper(
            nums: &Vec<i32>,
            used: &mut Vec<bool>,
            start: usize,
            k: i32,
            current_sum: i32,
            target: i32,
        ) -> bool {
            if k == 0 {
                return true; // All subsets are formed
            }
            
            if current_sum == target {
                return helper(nums, used, 0, k - 1, 0, target); // Start new subset
            }
            
            for i in start..nums.len() {
                if !used[i] && current_sum + nums[i] <= target {
                    used[i] = true;
                    if helper(nums, used, i + 1, k, current_sum + nums[i], target) {
                        return true;
                    }
                    used[i] = false; // Backtrack
                }
            }
            
            false
        }

        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }

        let target = sum / k;
        let mut nums = nums.clone();
        // nums.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending for optimization
        let mut used = vec![false; nums.len()];

        helper(&nums, &mut used, 0, k, 0, target)
    }
}
