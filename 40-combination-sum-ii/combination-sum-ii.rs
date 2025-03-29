impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        fn helper(
            candidates: &Vec<i32>,
            target: i32,
            result: &mut Vec<Vec<i32>>,
            curr: &mut Vec<i32>,
            curr_sum : i32,
            mut ind: usize
        ){
            if target == curr_sum{
                result.push(curr.clone());
            }else if ind<candidates.len() && target>curr_sum{

                curr.push(candidates[ind]);
                helper(
                    candidates, 
                    target, 
                    result, 
                    curr, 
                    curr_sum + candidates[ind], 
                    ind+1
                    );
                curr.pop();
                
                while ind+1< candidates.len() && candidates[ind+1] == candidates[ind]{
                    ind+=1;
                }

                helper(
                    candidates, 
                    target, 
                    result, 
                    curr, 
                    curr_sum, 
                    ind+1
                    );
            }
        }

        let mut result = Vec::new();
        let mut curr = Vec::new();
        let mut candidates = candidates.clone();
        candidates.sort();
        helper(
            &candidates,
            target,
            &mut result,
            &mut curr,
            0,
            0
        );
        result
        
    }
}