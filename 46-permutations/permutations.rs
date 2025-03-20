impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {

        fn solve(nums: &mut Vec<i32> , l : usize, r: usize, result: &mut Vec<Vec<i32>>){

            if l == r {

                result.push(nums.clone());
                return ;

            }

            for i in l..=r{

                nums.swap(l, i);
                solve(nums, l+1, r, result);
                nums.swap(l, i);

            }

        }



        let mut results = Vec::new();
        let len = nums.len();
        if len > 0 {

            solve(&mut nums, 0 , len-1, &mut results);

        }

        results
        

        
    }
}