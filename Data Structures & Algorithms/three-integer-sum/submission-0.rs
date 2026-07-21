use std::collections::HashSet;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut set = HashSet::new();
        for i in 0..(nums.len() - 2) {
            println!("i = {i}");
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                if nums[i] + nums[j] + nums[k] == 0 {
                    set.insert(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;
                } else if nums[i] + nums[j] + nums[k] > 0 {
                    k -= 1;
                } else {
                    j += 1;
                }
            }
        }

        set.into_iter().collect()
    }
}
