impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut vec_u8 = vec![];
        for c in s.chars() {
            match c as u8 {
                val@97..=122 => vec_u8.push(val),
                val@65..=90 => vec_u8.push(val + 32),
                val@48..=57 => vec_u8.push(val),
                _ => {}
            } 
        }

        if vec_u8.is_empty() { return true; }

        let mut start = 0;
        let mut end = vec_u8.len() - 1;
        while start < end {
            if vec_u8[start] == vec_u8[end] {
                start += 1;
                end -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
