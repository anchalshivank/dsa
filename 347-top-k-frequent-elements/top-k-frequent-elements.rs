use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;
#[derive(Eq, PartialEq, Debug)]
struct Entry{
    frequency: i32,
    value: i32
}

impl Entry{

    fn new(value: i32, frequency: i32) -> Self{

        Self{
            value, 
            frequency
        }

    }

}

impl Ord for Entry{

    fn cmp(&self, other: &Self) -> Ordering{

        other.frequency.cmp(&self.frequency)

    }

}

impl PartialOrd for Entry{

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        Some(self.cmp(other))


    }

}


impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut pq : BinaryHeap<Entry> = BinaryHeap::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        //the priority shall be based on the frequence 


        //now 



        for (index, num) in nums.iter().enumerate(){

            
            let freq = map.entry(*num).or_insert(0);
            *freq +=1;



        }

        //now we need to deal with prioriy queye

        for (&value, &frequency) in map.iter(){

                // println!("{:?}", pq);

            let entry = Entry::new(value, frequency);
            
            pq.push(entry);

            if pq.len() > k as usize {

                pq.pop();

            }


        }

        

        pq.into_iter().map(|entry| entry.value).collect()

    
        
    }
}