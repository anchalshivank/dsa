impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        
        let square = |x| x*x;

        let mut i : i32=0 ;
        let mut j : i32 = nums.len() as i32 -1;
        //now I know that this array is sorted starting can be negative or positive 
        //I will compare them .. if and but the bigger one at the back

        let mut result = vec![0; nums.len()];
        let mut k: i32 = result.len() as i32 -1;

        while k >=0 && i < nums.len() as i32  && j >= i{
            // println!("{i} {j} {k}");
            let left = square(nums[i as usize]);
            // println!("{j}");
            let right = square(nums[j as usize]);

            if left > right{
                result[k as usize] = left;
                i+=1;
            }else{
                result[k as usize] = right;
                j-=1;
            }

            k-=1;


        }

        result

    }
}