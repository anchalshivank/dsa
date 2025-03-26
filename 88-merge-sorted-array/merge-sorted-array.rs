impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        
        let (mut m, mut n) = (m as usize, n as usize);
        let mut k = m+n-1;

        while n > 0 {

            if m > 0 && nums1[m-1] > nums2[n-1]{

                nums1[k] = nums1[m-1];
                m-=1;
                k-=1;

            }else{

                nums1[k] = nums2[n-1];
                n-=1;
                k-=1;

            }

        }



    }
}