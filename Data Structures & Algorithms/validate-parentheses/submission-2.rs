impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 { return false; }
        let mut stack = vec![];
        for c in s.chars() {
            if c == '(' || c == '{' || c == '[' {
                stack.push(c);
            } else {
                if let Some(v) = stack.pop() {
                    match (v, c) {
                        ('(', ')') => {}
                        ('[', ']') => {}
                        ('{', '}') => {}
                        (_, _) => {return false;}
                    }
                } else {
                    return false;
                }
            }
        }
        if !stack.is_empty() { return false;}
        return true;
    }
}
