pub struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let mut result = 0;
        let mut roman_hash_map = HashMap::new();
        let mut symbols = vec!['I', 'V', 'X', 'L', 'C', 'D', 'M'];
        let mut values = vec![1, 5, 10, 50, 100, 500, 1000];
        roman_hash_map = symbols.into_iter().zip(values.into_iter()).collect();
        for (index, character) in s.chars().into_iter().enumerate() {
            if s[index] == 'V' && s[index - 1] == 'I' {
                result += 4
            } else if s[index] == 'X' && s[index - 1] == 'I' {
                result += 9
            } else if s[index] == 'L' && s[index - 1] == 'X' {
                result += 40
            } else if s[index] == 'C' && s[index - 1] == 'I' {
                result += 90
            } else if s[index] == 'D' && s[index - 1] == 'C' {
                result += 400
            } else if s[index] == 'X' && s[index - 1] == 'I' {
                result += 900
            } else {
                result += roman_hash_map.get(&character).copied().unwrap_or(0);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let test_case = "III".to_string();
        let result = Solution::roman_to_int(test_case);
        assert_eq!(result, 3);
    }
}
