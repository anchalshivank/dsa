impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        fn helper(
            nums: &Vec<i32>,
            curr: &mut Vec<i32>,
            mut ind: usize,
            result: &mut Vec<Vec<i32>>
        ){

            if ind == nums.len(){

                result.push(curr.clone());

            }else{

                //we can choose cuirrent or move

             
                curr.push(nums[ind]);
                helper(nums, curr, ind+1, result);
                curr.pop();
                   while ind+1 < nums.len() && nums[ind] ==nums[ind+1]{
                    ind+=1;
                }

                helper(nums, curr, ind+1, result);
            }


        }

        let mut result = Vec::new();

        let mut curr = Vec::new();
        let mut nums = nums.clone();
        nums.sort();
        helper(&nums, &mut curr, 0 , &mut result);
        result

    }
}