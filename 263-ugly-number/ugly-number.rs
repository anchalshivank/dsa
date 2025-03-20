impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        
        let  mut m = n;

        while m > 0{
            //divisible by 5
            if m%5 == 0{
                //then divide 
                m = m/5;
            }else if m%3 ==0{
                m = m/3;
            }else if m%2 == 0{
                m = m/2;
            }else{
                //in this case it can be divisible by any other number
                //if m !=1 than not a ugly number
                break;
            }

        }

        m == 1





    }
}