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

            *s1_map.entry(ch).or_insert(0)+=1;


        }

        //do the same for s2

        let s2_chars: Vec<char> = s2.chars().collect();

        for i in 0..n{

            *s2_map.entry(s2_chars[i]).or_insert(0)+=1;


        }

        //now compare if these both maps are equal

        if s1_map == s2_map{

            return true;

        }

        //we might find the sliding thing in next part

        for i in n..m{

            let to_add = s2_chars[i];
            let to_rem = s2_chars[i-n];
         
            *s2_map.entry(to_add).or_insert(0) +=1;
            *s2_map.entry(to_rem).or_insert(0) -=1;

            //Now incase to_rem beomes zero we will remoge th key also 

            if s2_map[&to_rem] == 0{

                s2_map.remove(&to_rem);

            }

            //compare the map now.
            if s2_map == s1_map{
                return true;
            }

        }

        false

    }
}