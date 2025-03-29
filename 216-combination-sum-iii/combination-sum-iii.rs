impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        
        fn helper(

            curr: &mut Vec<i32>,
            curr_sum: i32,
            ind: i32,
            result: &mut Vec<Vec<i32>>,
            target: i32,
            k: i32
        ){

            if target == curr_sum && curr.len() as i32 == k {
                result.push(curr.clone());
            }else if curr_sum < target && ind < 10 && (curr.len() as i32) < k {

                //choose curr or leave
                curr.push(ind);
                helper(
                    curr,
                    curr_sum+ ind,
                    ind+1,
                    result,
                    target,
                    k
                );
                curr.pop();
                //skip current
                helper(
                    curr,
                    curr_sum,
                    ind+1,
                    result, 
                    target,
                    k
                );
            }


        }

        let mut result = Vec::new();
        let mut curr = Vec::new();

        helper(
            &mut curr,
            0,
            1,
            &mut result,
            n,
            k
        );
        result

    }
}