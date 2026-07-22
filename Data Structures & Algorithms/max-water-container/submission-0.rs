impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = heights.len() - 1;
        let mut max_area = 0;
        while i < j {
            max_area = max_area.max(heights[i].min(heights[j]) * (j - i) as i32);
            if heights[i] < heights[j] { i += 1;}
            else if heights[i] == heights[j] { i += 1; j -= 1;}
            else { j -= 1;}
        }
        return max_area;
    }
}
