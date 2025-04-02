impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        
        let mut prod = 1;
        let mut max = i32::MIN;
        for (index, &num) in nums.iter().enumerate(){

            prod = prod*num;
            max = max.max(prod);
            if prod == 0{
                prod = 1;
            }




        }

        prod = 1;

        for &num in nums.iter().rev(){

            prod *=num;
            max = max.max(prod);

            if prod == 0{
                prod = 1;
            }

        }

        return max
        
        

    }
}