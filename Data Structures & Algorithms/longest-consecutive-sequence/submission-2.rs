impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        nums.sort();

        let mut highest_consecutive = 1;
        let mut consecutive = 1;
        for idx in 1..nums.len() {
            if nums[idx] - 1 == nums[idx-1] {
                consecutive += 1;
            } else if nums[idx] == nums[idx - 1] { continue;}
            else {
                highest_consecutive = highest_consecutive.max(consecutive);
                consecutive = 1;
            }
        }
        highest_consecutive = highest_consecutive.max(consecutive);

        highest_consecutive
    }
}
