impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut flag = false;

        let mut count = 0;
        
        let product: i32 = nums.iter().filter(|&x| if *x == 0{
            flag = true;
            count+=1;
            false
        }else{
            true
        }).product();
        // println!("{count}");
        
        let mut result = Vec::new();
        for i in 0..nums.len(){
            if count > 1{
                result.push(0);
            }
            else if nums[i] == 0{
                result.push(product);
            }else{
                if flag{
                    result.push(0);
                }else{

                    result.push(product/nums[i]);
                }
            }

        }
        result
    }
}