use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // Convert to usize for indexing
        let n = num_courses as usize;

        // Adjacency list for the graph: course -> list of dependent courses
        let mut graph = vec![Vec::new(); n];

        // In-degree array: in_degree[i] = number of prerequisites for course i
        let mut in_degree = vec![0; n];

        // Build the graph and compute in-degrees
        for pair in prerequisites {
            let (course, prereq) = (pair[0] as usize, pair[1] as usize);
            graph[prereq].push(course);   // prereq -> course
            in_degree[course] += 1;
        }

        // Queue to perform BFS, starting with courses that have no prerequisites
        let mut queue = VecDeque::new();
        for i in 0..n {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        // Counter to track how many courses we can finish
        let mut finished = 0;

        // Perform BFS (topological sort)
        while let Some(course) = queue.pop_front() {
            finished += 1;

            // Reduce in-degree of all neighbors
            for &neighbor in &graph[course] {
                in_degree[neighbor] -= 1;

                // If in-degree becomes 0, it can now be taken
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        // If we finished all courses, return true
        finished == n
    }
}
