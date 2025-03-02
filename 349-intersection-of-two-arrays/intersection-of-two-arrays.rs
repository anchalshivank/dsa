use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        
        let mut set = HashSet::new();

        for num in  nums1{

            set.insert(num);

        }

        let mut set2 = HashSet::new();

        for num in nums2{

            if set.contains(&num){

                set2.insert(num);

            }

        }

        set2.into_iter().collect()

    }
}