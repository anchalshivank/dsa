use std::collections::HashMap;
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if trust.is_empty() && n == 1 {
            return 1;
        }



        let mut map = HashMap::new();
        let mut map2 = HashMap::new();
        for t in trust.iter(){

              map.entry(t[1])
            .and_modify(|val: &mut Vec<i32>| val.push(t[0]))
            .or_insert(vec![t[0]]);

            map2.entry(t[0])
            .and_modify(|val: &mut Vec<i32>| val.push(t[1]))
            .or_insert(vec![t[1]]);  

        }


        
        println!("{:?}", map);
        println!("{:?}", map2);
        
        for (&key, val ) in map.iter(){

            if val.len() == n as usize -1 {

                //But there must be stutation where .. they key shall not trust anyone!!
                if map2.get(&key).map_or(true, |v| v.is_empty()) {
                    return key;
                }

            }

        }


        -1




    }
}