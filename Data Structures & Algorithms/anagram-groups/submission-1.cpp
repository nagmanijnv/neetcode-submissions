class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        vector<vector<string>> grouped_anagram;
        if (strs.size() < 2) {
            grouped_anagram.push_back(strs);
            return grouped_anagram;
        }

        // O(m * nlog(n))
        // nlog(n) for sorting each string in worst case, as n is length of longest string
        // sorting m strings will be O(m * nlog(n))
        // others stuffs are just moving items from hashmap to vector again
        /*
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
        */


        // try using vector as key of hashmap and that vector can 
        // be of char count for each string
        // i.e for represting 1 string
        // [0..25] => this will store count of char from a to z
        std::map<vector<int>, vector<string>> grouped_map;
        for (int i = 0; i < strs.size(); i++) {
            vector<int> key(26);
            for (int j = 0; j < strs[i].length(); j++) {
                key[strs[i][j] - 'a']++;
            }
            if (grouped_map.find(key) == grouped_map.end()) {
                vector<string> vec;
                vec.push_back(strs[i]);
                grouped_map[key] = vec;
            } else {
                grouped_map.find(key)->second.push_back(strs[i]);
            }
        }
        for (auto vec: grouped_map) {
            grouped_anagram.push_back(vec.second);
        }
        return grouped_anagram;
    }
};
