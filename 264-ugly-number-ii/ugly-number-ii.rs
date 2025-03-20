use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {

        //we need  heap with top nth ugly number which means min heap to get the number
        //if current ugly number is less than the top of min heap .. than that needs to be changed
        //pushing will makes sense .. but poping will only pop the least element 

        let mut bp = BinaryHeap::new();
        let mut seen = HashSet::new();

        let mut un = 1 as i64;
        bp.push(Reverse(un));

        seen.insert(un);

        //it shall bne a min heap
        for _ in 0..n {

            if let Some(Reverse(val)) = bp.pop(){

                un = val;

                let next_vals = [2*val, 3*val, 5*val];
                for next in next_vals{
                    if seen.insert(next){
                        bp.push(Reverse(next));
                    }
                }

            }

        }

        un as i32


        
    }
}