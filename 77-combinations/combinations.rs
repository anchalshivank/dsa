impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        
        fn helper(
            i: i32, 
            n: i32, 
            k: i32,
            temp: &mut Vec<i32> , 
            result: &mut Vec<Vec<i32>> 
            )
        {

            

            if temp.len() == k as usize{

                result.push(temp.clone());
            }else if temp.len() < k as usize && i <= n {

                temp.push(i);

                helper(i+1, n , k , temp, result);

                temp.pop();

                helper(i+1, n , k , temp, result);

            }





        }

        let mut result = Vec::new();
        let mut temp = Vec::new();
        helper(
            1, 
            n,
            k , 
            &mut temp,
            &mut result

        );
        result



    }
}