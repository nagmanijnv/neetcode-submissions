impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            if right - left == 1 {
                if nums[right] > nums[left] { return nums[left]}
                else {return nums[right]}
            }
            let mid = left + (right - left + 1) / 2;
            if nums[mid] > nums[right] && nums[mid] > nums[left] {
                left = mid + 1;
            }
            // This case not possible if sorted asceding order item rotated
            // else if nums[mid] > nums[right] && nums[mid] < nums[left] {}
            else if nums[mid] < nums[right] && nums[mid] > nums[left] {
                right = mid - 1;
            }
            else {
                // nums[mid] is smaller than left as well right
                if nums[mid] < nums[mid-1] {
                    return nums[mid]
                } else {
                    right = mid - 1;
                }
            }
        }

        if left == right {
            return nums[left];
        }

        return -1;
    }
}
