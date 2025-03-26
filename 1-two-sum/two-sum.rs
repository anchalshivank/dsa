use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map = HashMap::new();

        for (index, num) in nums.iter().enumerate(){

            map.insert(num , index as i32);

        }

        for (index, num ) in nums.iter().enumerate(){
            let to_find = target-num;
            if map.contains_key(&to_find){
                let index2 = *map.get(&to_find).unwrap();
                if index as i32 !=index2{

                return vec![index2, index as i32];
                }
            }
        }

        vec![]
        
    }
}