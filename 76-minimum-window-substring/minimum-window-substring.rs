use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }

        let mut need = HashMap::new();
        for ch in t.chars() {
            *need.entry(ch).or_insert(0) += 1;
        }

        let (mut left, mut right) = (0, 0);
        let mut min_len = usize::MAX;
        let mut start = 0;
        let mut window = HashMap::new();
        let mut valid = 0;

        let s_chars: Vec<char> = s.chars().collect();

        while right < s_chars.len() {
            let ch = s_chars[right];
            right += 1;

            if need.contains_key(&ch) {
                *window.entry(ch).or_insert(0) += 1;
                if window[&ch] == need[&ch] {
                    valid += 1;
                }
            }

            while valid == need.len() {
                if right - left < min_len {
                    min_len = right - left;
                    start = left;
                }

                let remove = s_chars[left];
                left += 1;

                if need.contains_key(&remove) {
                    if window[&remove] == need[&remove] {
                        valid -= 1;
                    }
                    *window.entry(remove).or_insert(0) -= 1;
                }
            }
        }

        if min_len == usize::MAX {
            "".to_string()
        } else {
            s_chars[start..start + min_len].iter().collect()
        }
    }
}
