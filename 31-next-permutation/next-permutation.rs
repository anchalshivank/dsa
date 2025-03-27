impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {

        //if the array is strictly decreasing 
        //reverse it 
        //is ascneding 
        let mut i : i32= nums.len() as i32 -2;

        while i >=0 && nums[i as usize +1]<=nums[i as usize]{

            i-=1;

        }



        if i>=0{

            let mut j = nums.len()-1;
            while nums[j]<=nums[i as usize]{
                j-=1;
            }
            nums.swap(i as usize, j);

        }

        nums[(i as usize+1)..].reverse();

        
    }
}