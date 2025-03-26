impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        
        let mut maj = nums[0];

        let mut count = 1;

        for &num in nums[1..].iter(){

            if num == maj{
                count+=1;
            }else{
                count -=1;
            }
            // println!("{count}");
            if count == -1{
                maj = num;
                count = 1;
            }

        }

        maj

    }
}