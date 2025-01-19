impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let t1: Vec<char> = text1.chars().collect();
        let t2: Vec<char> = text2.chars().collect();

        let len1 = t1.len();
        let len2 = t2.len();

        // Use two arrays for DP: previous row and current row
        let mut prev = vec![0; len2 + 1];
        let mut current = vec![0; len2 + 1];

        for i in 0..len1 {
            for j in 0..len2 {
                if t1[i] == t2[j] {
                    // If characters match, increment the length from the diagonal (prev[j] + 1)
                    current[j + 1] = prev[j] + 1;
                } else {
                    // Otherwise, take the max of the left or top value
                    current[j + 1] = current[j].max(prev[j + 1]);
                }
            }
            // After finishing the current row, copy it to the prev row for the next iteration
            prev.copy_from_slice(&current);
        }

        // The result is in the last cell of the current row (current[len2])
        current[len2]
    }
}
