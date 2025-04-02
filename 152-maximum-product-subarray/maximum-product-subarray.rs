impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        
    
        let mut pre = 1;

        let mut suff = 1;
        let mut ans = i32::MIN;
        for i in 0..nums.len(){

            if pre == 0 {
                pre = 1;
            }
            if suff == 0 {
                suff = 1;
            }

            pre *=nums[i];
            suff *= nums[nums.len()-i - 1];

            ans = i32::max(ans, i32::max(pre, suff));

        }

        return ans

        



        
        

    }
}