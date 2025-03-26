use std::collections::HashSet;
use rand::thread_rng;
use rand::Rng;
use rand::prelude::IteratorRandom;
struct RandomizedSet {
    set: HashSet<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self{
            set: HashSet::new()
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        
        if self.set.contains(&val){
            return false;
        }else{
            self.set.insert(val);
            true
        }

    }
    
    fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        self.set.iter().choose(&mut rng).cloned().unwrap()
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */