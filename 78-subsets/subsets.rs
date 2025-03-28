impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        fn helper(
            nums: &Vec<i32> , 
            ind: i32, 
            curr: &mut Vec<i32>, 
            result: &mut Vec<Vec<i32>>
            ){

                if ind == nums.len() as i32{
                    result.push(curr.clone());
                }else{

                    //we can choose curr or skip
                    curr.push(nums[ind as usize]);
                    helper(nums, ind+1, curr, result);

                    curr.pop();
                    helper(nums, ind + 1, curr, result);

                }

            }

        let mut result = Vec::new();
        let mut curr = Vec::new();
        helper(&nums, 0 , &mut curr, &mut result);
        result

    }
}