impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {

        let (mut i , mut j , mut k ) = (0,0, nums.len()-1);

        while j <= k{

            match nums[j]{

                0 => {
                    nums.swap(i, j);
                    i+=1;
                    j+=1;

                }
                1=>{
                    j+=1;
                }
                2=>{

                    nums.swap(j, k);
                    if k == 0{

                        break;

                    }
                    k -=1;

                }
                _=>{}

            }


        }
        
    }
}