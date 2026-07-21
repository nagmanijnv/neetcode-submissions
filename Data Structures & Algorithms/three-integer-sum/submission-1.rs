use std::collections::HashSet;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        // instead of using HashSet, we can skip duplicate element to 
        // remove duplicates from the final result
        let mut result = Vec::new();
        let mut i = 0;
        while i < nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                if nums[i] + nums[j] + nums[k] == 0 {
                    result.push(vec![nums[i], nums[j], nums[k]]);
                    while j < k && nums[j] == nums[j+1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k-1] {
                        k -= 1;
                    }
                    j += 1;
                    k -= 1;
                } else if nums[i] + nums[j] + nums[k] > 0 {
                    k -= 1;
                } else {
                    j += 1;
                }
            }
            while i < nums.len() - 2 && nums[i] == nums[i+1] {
                i += 1;
            }
            i += 1;
        }

        // set.into_iter().collect()
        result
    }
}
