use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Num(i32);

impl PartialOrd for Num{

    fn partial_cmp(&self, other: &Num) -> Option<Ordering>{

            Some(other.cmp(&self))

    }

}

impl Ord for Num {


    fn cmp(&self, other: &Num) -> Ordering {

        self.0.cmp(&other.0)

    }

}


impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {

        //we need max heap but max heap means top will be always largest 
        // so we need min heap 

        let mut heap = BinaryHeap::new();



        for &num in nums.iter(){

            
            if heap.len() < k as usize{

                heap.push(Num(num));
            }else{

                if let Some(val) = heap.peek(){

                    if val.0 < num {

                        heap.pop();
                        heap.push(Num(num));
                    }

                }

            }

                

        }

       heap.pop().unwrap().0
        
    }
}