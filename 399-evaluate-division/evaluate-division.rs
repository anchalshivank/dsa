use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut map = HashMap::new();

        for (index, eqn) in equations.iter().enumerate() {
            let a = &eqn[0];
            let b = &eqn[1];
            let val = values[index];

            map.entry(a.clone())
                .or_insert_with(Vec::new)
                .push((b.clone(), val));
            map.entry(b.clone())
                .or_insert_with(Vec::new)
                .push((a.clone(), 1.0 / val));
        }

        let mut result = Vec::new();

        for (i, query) in queries.iter().enumerate() {
            let start = &query[0];
            let target = &query[1];

            if !map.contains_key(start) || !map.contains_key(target) {
                result.push(-1.0);
                continue;
            }

            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            let mut found = false;

            queue.push_back((start, 1.0));

            while let Some((curr, prod)) = queue.pop_front() {
                if curr == target {
                    result.push(prod);
                    found = true;
                    break;
                }

                visited.insert(curr);

                if let Some(neighbors) = map.get(curr) {
                    for (next, val) in neighbors {
                        if !visited.contains(next) {
                            queue.push_back((next, prod * val));
                        }
                    }
                }
            }

            if !found {
                result.push(-1.0);
            }
        }

        result
    }
}
