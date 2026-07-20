use std::collections::HashMap;
impl Solution {
    // solution using 4 bits separation and encoding using a to o
    // and using = as separator
    // Complexity is more than expected

    // fn map_4bits_to_char(val: u8) -> char {
    //     (97 + val) as char
    // }

    // pub fn encode(strs: Vec<String>) -> String {
    //     let mut encoded_str = String::new();
    //     for s in strs.iter() {
    //         for char in s.chars() {
    //             let high_nibble = char as u8 >> 4;
                
    //             // 2. Extract the lower 4 bits (e.g., 0001 -> 1)
    //             let low_nibble = char as u8 & 0x0F;
    //             encoded_str.push(Solution::map_4bits_to_char(high_nibble));
    //             encoded_str.push(Solution::map_4bits_to_char(low_nibble));
    //         };

    //         encoded_str.push('=');
    //     };

    //     encoded_str
    // }

    // pub fn decode(s: String) -> Vec<String> {
    //     if s.is_empty() { return vec![] }
    //     let mut splited_str = s.split('=').collect::<Vec<&str>>();
    //     splited_str.pop();
    //     splited_str.into_iter().map(|str| {
    //         let chars = str.chars().collect::<Vec<char>>();
    //         let val = chars.chunks(2).map(|pair| {
    //             let high_nibble = (pair[0] as u8 - 97) << 4;
    //             let low_nibble = (pair[1] as u8 - 97) & 0x0F;
    //             high_nibble | low_nibble
    //         })
    //         .collect::<Vec<u8>>();

    //         String::from_utf8_lossy(&val).to_string()
    //     })
    //     .collect::<Vec<String>>()
    // }

    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded_str = String::new();

        let mut str_concat = String::new();
        let mut len_concat_with_comma = vec![];
        for str in strs.iter() {
            str_concat.push_str(str);
            len_concat_with_comma.push(format!("{}", str.len()));
        }

        encoded_str.push_str(&len_concat_with_comma.join(","));
        encoded_str.push('=');
        encoded_str.push_str(&str_concat);
        encoded_str
    }

    pub fn decode(s: String) -> Vec<String> {
        let v: Vec<&str> = s.splitn(2, '=').collect();
        let splitted_len_vec = v[0]
        .split(',')
        .collect::<Vec<&str>>();

        if splitted_len_vec.contains(&"") { return vec![]}

        let len_vec: Vec<usize> = splitted_len_vec
            .iter()
            .map(|val| val.parse::<usize>().unwrap())
            .collect();

        let actual_str = v[1];

        let mut result = vec![];
        let mut start = 0;
        for len in len_vec {
            result.push(actual_str[start..start + len].to_string());
            start += len;
        }

        result
    }
}
