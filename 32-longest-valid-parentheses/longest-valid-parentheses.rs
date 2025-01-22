impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        
        let mut stack : Vec<i32>= Vec::new();
        let mut max_len = 0;
        stack.push(-1);
        let chars: Vec<char> = s.chars().collect(); 
        for (index, &ch) in  chars.iter().enumerate(){

            if ch == '('{
                stack.push(index as i32);
            }else{
                stack.pop();

                if stack.is_empty(){
                    stack.push(index as i32);
                }

                max_len = max_len.max(index as i32 - stack.last().unwrap());
            }


        }
        max_len
    }
}