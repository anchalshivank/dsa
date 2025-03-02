use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
struct Num(i64);

impl PartialOrd for Num {

    fn partial_cmp(&self, other: &Num) -> Option<Ordering> {

        Some(self.cmp(&other))

    }

}

impl Ord for Num {

     fn cmp(&self, other: &Num) -> Ordering {

        other.0.cmp(&self.0)

     }

}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        
        let mut bp = BinaryHeap::new();

        let mut seen = HashSet::new();
        let mut un = 1;

        bp.push(Num(un));
        seen.insert(un);

        for _ in 0..n {

            if let Some(val) = bp.pop(){

                un = val.0;

                let next_vals = [2*val.0, 3*val.0, 5*val.0];
                for next in next_vals{

                    if seen.insert(next){

                        bp.push(Num(next));

                    }

                }

            }

        }

        un as i32

    }
}