impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        

        let mut max_len = i32::MIN;
        let n = nums1.len();
        let m = nums2.len();
        let mut prev = vec![0; n+1];
        let mut curr = vec![0; n+1];
        
        for i in 1..=m{
            for j in 1..=n{

                if nums1[j-1] == nums2[i-1]{
                    curr[j] = prev[j-1] + 1;
                    max_len = max_len.max(curr[j]);
                }else{
                    curr[j] = 0;
                }

            }

            std::mem::swap(&mut prev, &mut curr);

        }
        if max_len == i32::MIN {0}else{max_len}

    }
}