impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        //first let's sort the thing

        nums.sort();
        let mut result = Vec::new();
        //let it be two pointer 
        let n = nums.len();
        for i in 0..n{

            if i > 0 && nums[i] == nums[i-1]{

                continue;

            }

            let mut j = i+1;
            let mut k = n-1;

            while j < k {

                let curr = nums[i]+nums[j]+nums[k];

                if curr == 0{
                    result.push(vec![nums[i], nums[j], nums[k]]);
                    while j < k && nums[j] == nums[j+1]{

                        j+=1;

                    }
                    while j < k && nums[k] == nums[k-1]{

                        k-=1;

                    }
                    j+=1;
                    k-=1;
                }else if curr > 0{
                    k-=1;
                }else{
                    j+=1;
                }

            }

        }

        result



    }
}