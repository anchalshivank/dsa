impl Solution {
    pub fn fib(n: i32) -> i32 {
        
        match n {
            0|1=>n,
            n => {
                let mut fn_2 = 0;
                let mut fn_1 = 1;
                for curr in 2..(n+1){
                    let f_n = fn_1+ fn_2;
                    fn_2 = fn_1;
                    fn_1 = f_n;



                }

                fn_1
            }
        }
        
        


    }
}