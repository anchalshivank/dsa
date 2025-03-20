use std::collections::HashMap;


impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        
        let n = s1.len();
        let m = s2.len();

        if n > m {

            return false;

        }

        let mut s1_map = HashMap::new();
        let mut s2_map = HashMap::new();

        for ch in s1.chars(){

            *s1_map.entry(ch).or_insert(0) +=1;

        }

        let s2_chars: Vec<char> = s2.chars().collect();

        for i in 0..n{

            *s2_map.entry(s2_chars[i]).or_insert(0) +=1;


        }

        if s1_map == s2_map{
            return true;
        }

        for i in n..m{

            let new_char = s2_chars[i];
            let old_char = s2_chars[i-n];

            *s2_map.entry(new_char).or_insert(0)+=1;
            *s2_map.entry(old_char).or_insert(0)-=1;

            if s2_map[&old_char] == 0 {

                s2_map.remove(&old_char);

            }

            if s1_map == s2_map{
                return true;
            }


        }

        false

    }
}