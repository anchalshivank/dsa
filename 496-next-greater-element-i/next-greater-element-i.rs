use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
        let mut stack = Vec::new();
        let mut map = HashMap::new();
        for (index, &num) in nums2.iter().rev().enumerate(){
            
            while let Some(&val) = stack.last(){
                if val <=num{
                    //do nothing
                    stack.pop();
                }else{
                    map.insert(num , val);
                    break;
                }
            }
            *map.entry(num).or_insert(-1);
            stack.push(num);            
        }

        // println!("{:?}", map);
        nums1.iter().map(|num| *map.get(num).unwrap()).collect()


    }
}