impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        
        let mut prev = 0;
        for curr in 1..nums.len(){
            if nums[curr] != nums[prev]{
                prev+=1;
                nums[prev] = nums[curr];
            }
        }

        (prev+1) as i32

    }
   

}