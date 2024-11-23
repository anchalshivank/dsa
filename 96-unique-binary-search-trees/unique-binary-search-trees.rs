impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut catalan: i64 = 1;

        for i in 0..n {
            // Ensure the result of the multiplication is cast to i64
            catalan = catalan * (2 * (2 * i + 1)) as i64 / (i + 2) as i64;
        }

        catalan as i32
    }
}
