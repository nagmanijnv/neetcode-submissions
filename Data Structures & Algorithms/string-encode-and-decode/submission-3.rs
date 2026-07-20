use std::collections::HashMap;
impl Solution {
    fn map_4bits_to_char(val: u8) -> char {
        (97 + val) as char
    }

    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded_str = String::new();
        for s in strs.iter() {
            for char in s.chars() {
                let byte  = char as u8;
                let high_nibble = byte >> 4;
                
                // 2. Extract the lower 4 bits (e.g., 0001 -> 1)
                let low_nibble = byte & 0x0F;
                encoded_str.push(Solution::map_4bits_to_char(high_nibble));
                encoded_str.push(Solution::map_4bits_to_char(low_nibble));
            };

            encoded_str.push('=');
        };

        encoded_str
    }

    pub fn decode(s: String) -> Vec<String> {
        if s.is_empty() { return vec![] }
        let mut splited_str = s.split('=').collect::<Vec<&str>>();
        splited_str.pop();
        splited_str.into_iter().map(|str| {
            let chars = str.chars().collect::<Vec<char>>();
            let val = chars.chunks(2).map(|pair| {
                let high_nibble = (pair[0] as u8 - 97) << 4;
                let low_nibble = (pair[1] as u8 - 97) & 0x0F;
                high_nibble | low_nibble
            })
            .collect::<Vec<u8>>();

            String::from_utf8_lossy(&val).to_string()
        })
        .collect::<Vec<String>>()
    }
}
