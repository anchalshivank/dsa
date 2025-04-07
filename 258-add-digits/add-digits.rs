impl Solution {
    pub fn add_digits(num: i32) -> i32 {

        if num%10 == num{
            return num;
        }

        let mut ans = 0;

        let mut n = num;

        while n > 0{
            ans +=n%10;
            n = n/10;

        }
        Self::add_digits(ans)
        //Now this ans can be !!!! double digit
        
    }
}