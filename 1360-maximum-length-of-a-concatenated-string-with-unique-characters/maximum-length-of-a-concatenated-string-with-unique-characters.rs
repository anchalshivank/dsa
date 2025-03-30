impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {

        fn helper(

            arr: &Vec<Vec<char>> ,
            idx: usize,
            curr: i32,
            freq: &mut Vec<usize>
        ) -> i32{

            if idx == arr.len(){

                //it means that 
                return curr; 
            }else{


                //choose curr or skip
                //and this is only possible if this curr string does note exists in curr string

                //can w
                if is_possible(freq, &arr[idx]){
                    //update the freq
                    let a  = helper(arr, idx+1, curr+ (arr[idx].len()) as i32 , freq);

                    roll_back(freq, &arr[idx]);
                    //we need to roll back freq

                    let b = helper(arr, idx+1, curr, freq);
                    return a.max(b);
                }else{
                    return helper(arr, idx+1, curr, freq);
                }
           
            }

        }
        
        fn roll_back(
            freq: &mut Vec<usize>,
            word: &Vec<char>
        ){

            for &ch in word.iter(){
                let idx = (ch as u8 - b'a') as usize;
                freq[idx]-=1;

            }

        }

        fn is_possible(
            freq: &mut Vec<usize>,
            word: &Vec<char>
        ) -> bool {
            
            //I want the current freq map
            //here
            let original_freq = freq.clone();

            for &ch in word.iter(){
                let idx = (ch as u8 - b'a') as usize;
                if freq[idx] > 0{
                    //before returning we need to roll back freq at the starting How to do that?? 
                    *freq = original_freq;
                    return false;

                }else{
                    freq[idx] = 1;
                }

            }
        
            
            //Now update the freq
            return true;
        }

        let mut freq = vec![0; 26];
        let arr : Vec<Vec<char>> = arr.iter().map(|word| word.chars().collect()).collect();
        helper(&arr, 0 , 0 , &mut freq)

        
    }
}