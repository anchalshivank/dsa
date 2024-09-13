use std::collections::HashSet;
use std::cmp::max;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set: HashSet<char> = HashSet::new();
        let mut i = 0;
        let mut len = 0;
        let mut j = 0;

        while j < s.len(){
            let c = s.chars().nth(j).unwrap();

            if !set.contains(&c){
                set.insert(c);
                j +=1;
                len = max(len, set.len());
            }else{
                let to_remove = s.chars().nth(i).unwrap();
                set.remove(&to_remove);
                i +=1;
            }
        }
        len as i32
    }
}