class Solution {
public:
    bool hasDuplicate(vector<int>& nums) {
        std::sort(nums.begin(), nums.end());
        int count = 0;
        for(int i = 1; i < nums.size(); i++) {
            if (nums[i-1] == nums[i]) {
                return true;
            }
        }
        return false;
    }
};