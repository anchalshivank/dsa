use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let n = nums.len();

        let mut map = HashMap::new();

        for (i , &num) in nums.iter().enumerate(){
            map.insert(num, i );
        }


        for (i , &num) in nums.iter().enumerate(){
            let complement = target-num;
            if let Some(&j) = map.get(&complement){
                if i!=j{
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]

        
        
    }
}