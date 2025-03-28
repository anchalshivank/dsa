impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        
        let mut i = 0;
        let mut j = nums.len()-1;

        while i<=j{

            let mid = i+ (j-i)/2;
            if nums[mid] == target{
                return mid as i32;
            }
            
            if nums[mid] >=nums[i]{
                //meas aschending from i to mid
                //if target is in this range 
                if target >= nums[i] && target <nums[mid]{
                    j = mid-1;
                }else{
                    //it means target is not present in i to mid
                    i = mid+1;
                }
            }else{
                if target>nums[mid] && target <= nums[j]{
                    i = mid+1;
                }else{
                    j = mid-1;
                }
            }

        }

        -1

    }
}