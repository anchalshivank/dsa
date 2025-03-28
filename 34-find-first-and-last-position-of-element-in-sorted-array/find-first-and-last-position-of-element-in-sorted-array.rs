impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut i = 0 ;
        let mut j = nums.len() as i32-1;

        while i <= j {

            let mid = i+(j-i)/2;

            if nums[mid as usize] == target{
                //we need to traverse back and from to get the duplicate thing
                let mut end = mid ;
                let mut start = mid ;
                while end < nums.len() as i32 && nums[end as usize] == target{
                    end+=1;
                }

                while start >= 0 && nums[start as usize] == target{

                    start-=1;

                }

                return vec![start+1, end-1];

            }else if nums[mid as usize] > target{
                j = mid-1;
            }else{
                i = mid+1;
            }

        }

        vec![-1,-1]
        
    }
}