impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {

        let mut max_reach = 0;

        for (i , &num ) in nums.iter().enumerate(){

            if i > max_reach as usize{

                return false;

            }

            max_reach = max_reach.max(i as i32 + num);

            if max_reach as usize >= nums.len()-1{
                return true;
            }

        }
        false 
    }
}