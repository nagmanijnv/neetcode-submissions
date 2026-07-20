impl Solution {
    // this is valid when nums length > 2
    // if length = 1 then return the same element
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 { return nums }
        let mut prefix_product = vec![];
        let mut suffix_product = vec![];
        for (idx, num) in nums.iter().enumerate() {
            if idx == 0 {
                prefix_product.push(*num);
                suffix_product.push(nums[nums.len() - 1 - idx]);
            } else {
                prefix_product.push(prefix_product[idx - 1] * num);
                suffix_product.push(suffix_product[idx - 1] * nums[nums.len() - 1 - idx]);
            }
        }

        let mut result = vec![0; nums.len()];
        for (idx, num) in prefix_product.iter().enumerate() {
            if idx == 0 {
                result[idx] = suffix_product[nums.len() - 1 - (idx + 1)];
            } else if idx == nums.len() -1 {
                result[idx] = prefix_product[idx - 1];
            } else {
                result[idx] = suffix_product[nums.len() - 1 - (idx + 1)] * prefix_product[idx - 1];
            }
        }

        result
    }
}
