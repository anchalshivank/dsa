impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(
            nums: &Vec<i32>,          // Reference to the sorted array (immutable)
            visited: &mut Vec<bool>,  // Tracks which elements are included in the current permutation
            curr: &mut Vec<i32>,      // Current permutation being built
            result: &mut Vec<Vec<i32>> // Stores all valid permutations
        ) {
            // If the current permutation is complete, add it to the result
            if curr.len() == nums.len() {
                result.push(curr.clone());
                return;
            }

            // Iterate over all elements in the array
            for i in 0..nums.len() {
                // Skip if the element is already visited
                if visited[i] {
                    continue;
                }

                // Skip duplicates: Only process the first occurrence of a duplicate group
                if i > 0 && nums[i] == nums[i - 1] && !visited[i - 1] {
                    continue;
                }

                // Mark the current element as visited
                visited[i] = true;

                // Add the current element to the permutation
                curr.push(nums[i]);

                // Recursive call to build the rest of the permutation
                helper(nums, visited, curr, result);

                // Backtrack: Unset visited[i] and remove the last element
                visited[i] = false;
                curr.pop();
            }
        }

        // Sort the array to group duplicates together
        let mut nums = nums.clone();
        nums.sort();

        let n = nums.len();
        let mut result = Vec::new();
        let mut visited = vec![false; n];
        let mut curr = Vec::new();

        // Generate all distinct permutations
        helper(&nums, &mut visited, &mut curr, &mut result);

        result
    }
}