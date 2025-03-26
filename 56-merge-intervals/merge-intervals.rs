impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a,b| a[0].cmp(&b[0]));
        let mut result = Vec::new();
        let mut curr = intervals[0].clone();
        if intervals.len() == 1{
            result.push(curr);
        }else{
    for i in 1..intervals.len(){

                let interval = &intervals[i];
                if curr[1] < interval[0]{
                    //we need to break the interval else continue
                    result.push(curr.clone());
                    curr = interval.clone();
                }else{
                    curr[1] = curr[1].max(interval[1]);
                }

                if i == intervals.len()-1{
                    result.push(curr.clone());
                }


            }
        }
        


        result

    }
}