impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
           let mut result = Vec::new();
        if matrix.is_empty() || matrix[0].is_empty() {
            return result;
        }

        let (mut x_s, mut x_e, mut y_s, mut y_e) = (0, matrix[0].len(), 0, matrix.len());

        while x_s < x_e && y_s < y_e {
            // Traverse from left to right
            for k in x_s..x_e {
                result.push(matrix[y_s][k]);
            }
            y_s += 1;
            if y_s >= y_e { break; }

            // Traverse from top to bottom
            for k in y_s..y_e {
                result.push(matrix[k][x_e - 1]);
            }
            x_e -= 1;
            if x_s >= x_e { break; }

            // Traverse from right to left
            for k in (x_s..x_e).rev() {
                result.push(matrix[y_e - 1][k]);
            }
            y_e -= 1;
            if y_s >= y_e { break; }

            // Traverse from bottom to top
            for k in (y_s..y_e).rev() {
                result.push(matrix[k][x_s]);
            }
            x_s += 1;
        }

        result
    }
}