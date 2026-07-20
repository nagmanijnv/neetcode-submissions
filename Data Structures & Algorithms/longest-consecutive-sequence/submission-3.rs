impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        // Using sorting with O(nlog(n))
        // nums.sort();

        // let mut highest_consecutive = 1;
        // let mut consecutive = 1;
        // for idx in 1..nums.len() {
        //     if nums[idx] - 1 == nums[idx-1] {
        //         consecutive += 1;
        //     } else if nums[idx] == nums[idx - 1] { continue;}
        //     else {
        //         highest_consecutive = highest_consecutive.max(consecutive);
        //         consecutive = 1;
        //     }
        // }
        // highest_consecutive = highest_consecutive.max(consecutive);

        // highest_consecutive



        // Using Hashset
        use std::collections::HashSet;
        let hashset: HashSet<_> = nums.clone().into_iter().collect();
        let mut max_count = 0;
        let mut count = 0;

        for num in nums {
            if !hashset.contains(&(num-1)) {
                count = 1;
                let mut tmp_val = num + 1;
                while hashset.contains(&(tmp_val)) {
                    count += 1;
                    tmp_val += 1;
                }
                max_count = max_count.max(count);
            }
        }
        max_count
    }
}
