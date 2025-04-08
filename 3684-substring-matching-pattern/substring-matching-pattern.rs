impl Solution {
    pub fn has_match(s: String, p: String) -> bool {
        fn create_lps(pattern: &[char]) -> Vec<usize> {
            let mut lps = vec![0; pattern.len()];
            let mut len = 0;
            let mut i = 1;

            while i < pattern.len() {
                if pattern[i] == pattern[len] {
                    len += 1;
                    lps[i] = len;
                    i += 1;
                } else {
                    if len != 0 {
                        len = lps[len - 1];
                    } else {
                        lps[i] = 0;
                        i += 1;
                    }
                }
            }

            lps
        }

        fn kmp_search(text: &[char], pattern: &[char]) -> Vec<usize> {
            if pattern.is_empty() {
                return (0..=text.len()).collect(); // match at all positions
            }

            let lps = create_lps(pattern);
            let mut result = vec![];
            let mut i = 0;
            let mut j = 0;

            while i < text.len() {
                if text[i] == pattern[j] {
                    i += 1;
                    j += 1;
                }

                if j == pattern.len() {
                    result.push(i - j);
                    j = lps[j - 1];
                } else if i < text.len() && text[i] != pattern[j] {
                    if j != 0 {
                        j = lps[j - 1];
                    } else {
                        i += 1;
                    }
                }
            }

            result
        }

        let s_chars: Vec<char> = s.chars().collect();
        let parts: Vec<&str> = p.split('*').collect();
        let (prefix, suffix) = (parts[0], parts[1]);
        let prefix_chars: Vec<char> = prefix.chars().collect();
        let suffix_chars: Vec<char> = suffix.chars().collect();

        let prefix_matches = kmp_search(&s_chars, &prefix_chars);
        let suffix_matches = kmp_search(&s_chars, &suffix_chars);

        for &start in &prefix_matches {
            let prefix_end = start + prefix_chars.len();
            for &end in &suffix_matches {
                if end >= prefix_end {
                    return true;
                }
            }
        }

        false
    }
}
