impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        
        if nums.len() == 0 {
            return 0;
        }
        let mut dp = vec![0;nums.len() +1];

        dp[0] = 0;
        dp[1] = nums[0];

        for i in 1..nums.len(){
            let val = nums[i];
            dp[i+1] = dp[i].max(dp[i-1]+val);
        }
        return dp[nums.len()]
        // helper(0, &nums, 0, nums.len())
    }
}