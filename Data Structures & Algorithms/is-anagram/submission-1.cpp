class Solution {
public:
    bool isAnagram(string s, string t) {
        std::unordered_map<char, int> char_map;

        if (s.length() != t.length()) return false;

        for(int i = 0; i < s.length(); i++) {
            char_map[s[i]]++;
        }

        for(int i = 0; i < t.length(); i++) {
            if (char_map.find(t[i]) == char_map.end()) return false;
            else {
                if (char_map.find(t[i])->second == 0) return false;
                char_map[t[i]]--;
            }
        }
        return true;
    }
};
