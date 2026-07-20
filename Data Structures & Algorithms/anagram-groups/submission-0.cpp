class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        vector<vector<string>> grouped_anagram;
        if (strs.size() < 2) {
            grouped_anagram.push_back(strs);
            return grouped_anagram;
        }

        std::unordered_map<string, vector<string>> grouped_map;
        for (const auto& str: strs) {
            string key = str;
            std::sort(key.begin(), key.end());
            if (grouped_map.find(key) == grouped_map.end()) {
                vector<string> vec;
                vec.push_back(str);
                grouped_map[key] = vec;
            } else {
                grouped_map.find(key)->second.push_back(str);
            }
        }

        for (auto vec: grouped_map) {
            grouped_anagram.push_back(vec.second);
        }

        return grouped_anagram;
    }
};
