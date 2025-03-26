impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {

        let first = vec![1];
        // let prev = vec![1,1];
        let mut result = Vec::new();
        result.push(first);
        // result.push(prev);
        for len in 1..num_rows{
            let mut curr_row = vec![1];
            let prev = &result[len as usize - 1];
            for col in 1..len{

                curr_row.push(prev[col as usize -1]+ prev[col as usize]);
                
            }

            curr_row.push(1);
            result.push(curr_row);

            


        }

        result

    }
}