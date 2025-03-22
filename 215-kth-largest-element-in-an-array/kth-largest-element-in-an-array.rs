use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        
        //we need a min heap of size k 


        let mut heap = BinaryHeap::new();


        for num in nums.iter(){

            if heap.len() < k as usize {

            heap.push(Reverse(num));



            }else{

                if let Some(Reverse(top)) = heap.peek(){

                    if *top < num {
                        heap.pop();
                        heap.push(Reverse(num));
                    }

                }

            }

        }

        *heap.pop().unwrap().0


    }
}