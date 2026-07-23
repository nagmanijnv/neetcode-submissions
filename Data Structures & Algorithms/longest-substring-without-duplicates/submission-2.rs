impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();

        let mut i = 0;
        let char_str = s.chars().collect::<Vec<char>>();
        let mut max_count = 0;
        let mut count = 0;
        let mut last_found_idx = 0;
        while i < s.len() {
            if let Some(val) = map.get(&char_str[i]) {
                max_count = max_count.max(count);
                if *val > last_found_idx {
                    count = (i - val) as i32;
                    last_found_idx = *val;
                } else {
                    count = (i - last_found_idx) as i32;
                }
                
            } else {
                count += 1;
            }
            map.insert(char_str[i], i);
            i += 1;
        }
        max_count = max_count.max(count);
        max_count
    }
}
