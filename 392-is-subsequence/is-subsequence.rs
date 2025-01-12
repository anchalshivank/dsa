impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        
        let len1 = s.len();
        let len2 = t.len();

        let mut i = 0usize;
        let mut j = 0usize;
        if len2< len1{
            return false;
        }
        while i < len1 && j < len2 {

            if s.as_bytes()[i] == t.as_bytes()[j]{
                i+=1;
            }
            j+=1;
        }

        i == len1

    }
}