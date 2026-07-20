class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        std::unordered_map<int, int> twosum_map;
        vector<int> result;
        for (int i = 0; i < nums.size(); i++) {
            auto it = twosum_map.find(target - nums[i]);
            if (it == twosum_map.end()) {
                twosum_map.insert({nums[i], i});
            } else {
                int index = it->second;
                result.push_back(index);
                result.push_back(i);
                return result;
            }
        }
        // return 
    }
};
