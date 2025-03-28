impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        
        //
        let n = nums.len() as i32;
        let mut i = n-1;

        while i>0 && nums[i as usize -1]>=nums[i as usize]{
            i-=1;
        }

        if i==0{
            nums.reverse();
            return;
        }

        //Now this nums[i-1]<nums[i]

        //we will swap this element just greater than it.. it shall be againf rom back
        let mut j = n -1;
        while j > 0 && nums[j as usize]<=nums[i as usize-1]{

            //
            j-=1;
        }

        nums.swap(j as usize, i as usize-1);

        nums[(i as usize)..].reverse()

    }
}