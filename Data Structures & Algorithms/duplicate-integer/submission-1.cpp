class Solution {
public:
    bool hasDuplicate(vector<int>& nums) {
        // O(nlog(n)) complexity
        /*
        std::sort(nums.begin(), nums.end());
        int count = 0;
        for(int i = 1; i < nums.size(); i++) {
            if (nums[i-1] == nums[i]) {
                return true;
            }
        }
        return false;
        */

        // O(n) complexity => use hashmap
        std::unordered_map<int, int> nums_map;
        for (int i = 0; i < nums.size(); i++) {
            if (nums_map.find(nums[i]) == nums_map.end()) {
                nums_map[nums[i]] = 1;
            } else {
                return true;
            }
        }
        return false;
    }
};