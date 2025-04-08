impl Solution {
    pub fn longest_prefix(s: String) -> String {
        
        let chars : Vec<char> = s.chars().collect();

        // if s.len() == 1{
        //     return 
        // }

        let mut i = 0;
        let n = s.len();

        let mut arr = vec![0; n];

        //we shall always increase j
        let mut j = 1;
        while j < n{
            if chars[j] == chars[i]{

                //it means we can do msomething crazy
                arr[j] = i+1; 
                j+=1;
                i+=1;
            }else{
                //we found a mismatch // this means that there can be possibiliy of
                //find another short prefix
                if i  == 0{
                    j+=1;
                }else{
                    i = arr[i-1];    
                }
            }
        }
        if let Some(&e) = arr.last(){
            let e = e as usize;
            return s[..e].to_string();

        }
        "".to_string()



    }
}