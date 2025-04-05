use std::collections::{HashMap, VecDeque, HashSet};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_len = begin_word.len();
        let mut word_set: HashSet<String> = word_list.iter().cloned().collect();

        if !word_set.contains(&end_word) {
            return 0;
        }

        let mut all_combo_dict: HashMap<String, Vec<String>> = HashMap::new();

        // Create buckets of wildcard combinations
        for word in &word_list {
            for i in 0..word_len {
                let pattern = format!("{}*{}", &word[0..i], &word[i+1..]);
                all_combo_dict
                    .entry(pattern)
                    .or_default()
                    .push(word.clone());
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back((begin_word.clone(), 1));

        let mut visited = HashSet::new();
        visited.insert(begin_word);

        while let Some((current_word, level)) = queue.pop_front() {
            for i in 0..word_len {
                let pattern = format!("{}*{}", &current_word[0..i], &current_word[i+1..]);
                if let Some(neighbors) = all_combo_dict.get(&pattern) {
                    for neighbor in neighbors {
                        if neighbor == &end_word {
                            return level + 1;
                        }
                        if !visited.contains(neighbor) {
                            visited.insert(neighbor.clone());
                            queue.push_back((neighbor.clone(), level + 1));
                        }
                    }
                }
            }
        }

        0
    }
}
