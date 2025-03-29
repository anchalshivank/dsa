impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {

        fn solve(
            index: usize,
            digits: &Vec<char>,
            comb : &[&str],
            ans: &mut Vec<String>,
            temp: String
        ){

            if index == digits.len(){

                ans.push(temp);
                return;
            }

            let digit = digits[index].to_digit(10).unwrap() as usize;
            // println!("{digit}");

            for ch in comb[digit].chars(){
                solve(index+1, digits, comb, ans, format!("{}{}", temp, ch));
            }


        }

        if digits.is_empty(){
            return vec![];
        }
        let digits : Vec<char> = digits.chars().collect();
        let comb = ["","","abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
        let mut ans = Vec::new();
        solve(0, &digits, &comb, &mut ans, String::new());
        ans
      
    }
}