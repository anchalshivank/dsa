impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {

        let mut ans = i32::MAX;

        //first calculate the smallest element on the right side of the element and smallet elemtn on the leftt side 
        let n = nums.len();
        let mut min_l = vec![nums[0]; n];
        let mut min_r = vec![nums[n-1]; n];

        for i in 1..n{

            min_l[i] = min_l[i-1].min(nums[i]);

        }


        for i in (0..n-1).rev(){

            min_r[i] = min_r[i+1].min(nums[i]);

        }

         for i in 1 .. n - 1 {
      if nums[i] > min_l[i - 1] && nums[i] > min_r[i + 1] {
        ans = ans.min(nums[i] + min_l[i - 1] + min_r[i + 1]);
      }
    }
        if ans==i32::MAX{
            -1
        }else{
            ans
        }

        
    }
}