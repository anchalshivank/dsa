impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        
        let mut n = x;

        let mut a = 0;
        while n > 0{

            let rem = n%10;
            a = a*10+ rem;
            n = n/10;
            

        }

        a==x


    }
}