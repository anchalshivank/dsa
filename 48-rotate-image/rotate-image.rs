impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {

        let n = matrix.len();

        // Iterate through each layer
        for layer in 0..n / 2 {
            let xs = layer;         // Starting x (left)
            let xe = n - 1 - layer; // Ending x (right)
            let ys = layer;         // Starting y (top)
            let ye = n - 1 - layer; // Ending y (bottom)

            // Rotate elements within the layer
            for off in 0..(xe - xs) {
                // Store top element
                let temp = matrix[ys][xs + off];

                // Move left -> top
                matrix[ys][xs + off] = matrix[ye - off][xs];

                // Move bottom -> left
                matrix[ye - off][xs] = matrix[ye][xe - off];

                // Move right -> bottom
                matrix[ye][xe - off] = matrix[ys + off][xe];

                // Move top -> right
                matrix[ys + off][xe] = temp;
            }
        }
    }
}