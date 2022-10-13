// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// How to think?
// We create hashmap and two pointers , one will work as pointer to the longest substring found , and the other will work as index
// We loop throught the string and check insertion of each character into the hashmap , if inserted, it means that it is unique
// so we increate the first pointer (longest_substring_lenght_found) if not inserted we compare the current_index with longest_substring_lenght_found
// and return max 
pub struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut longest_substring_lenght_found = -1;

        let mut max_length = 0;
        let mut values: HashMap<char, i32> = HashMap::new();
        for (current_index, char) in s.chars().enumerate() {
            if let Some(current_index) = values.insert(char, current_index as i32) {
                longest_substring_lenght_found = std::cmp::max(longest_substring_lenght_found, current_index);
            }
            max_length = std::cmp::max(max_length, current_index as i32 - longest_substring_lenght_found )
            }
        max_length

    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_longest_substring() {
        let test_case = "pwwkew";
        let ans = Solution::length_of_longest_substring(test_case.to_string());
        assert_eq!(ans, 3);
    }
}
