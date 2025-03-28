use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        

        let mut map = HashMap::new();
        map.insert(0,1);

        let mut curr_sum = 0;
        let mut count = 0;

        for num in nums{

            curr_sum +=num;

            let mut remainder = curr_sum % k;

            if remainder < 0 {

                remainder +=k;

            }

            if let Some(&freq) = map.get(&remainder){

                count+=freq;

            }

            *map.entry(remainder).or_insert(0)+=1;


        }

        count

    }
}