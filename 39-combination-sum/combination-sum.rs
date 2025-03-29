impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        fn helper(

            curr: &mut Vec<i32>,
            target: i32,
            curr_sum: i32,
            ind: usize,
            candidates: &Vec<i32>,
            result: &mut Vec<Vec<i32>>


        ){
            if curr_sum == target{

                result.push(curr.clone());

            }else if ind< candidates.len() && curr_sum < target{
                //
                curr.push(candidates[ind]);
                helper(
                    curr, 
                    target,
                    curr_sum+candidates[ind],
                    ind,
                    candidates,
                    result
                );

                curr.pop();

                helper(
                    curr, 
                    target,
                    curr_sum,
                    ind + 1,
                    candidates,
                    result
                );

            }

        }

        let mut result = Vec::new();
        let mut curr = Vec::new();
        helper(
            &mut curr,
            target, 
            0 , 
            0,
            &candidates,
            &mut result
        );

        result
        
    }
}