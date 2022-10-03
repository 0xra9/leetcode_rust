use std::sync::mpsc::channel;

// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
pub struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        // Declaring empty stack(vector) to push/pop findings from it.
        let mut stack = Vec::new();
        // looping through the argument String character by character
        for char in s.chars() {
            match char {
                '{'  => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                // stack.pop() returns Option , so we must use Some to get the value from the option.
                '}' | ')' | ']' if Some(char) != stack.pop() => return false,
                _ => (),
            }
        }
        return stack.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_nested_vps_depth() {
        let test_case = String::from("([)]");
        let ans = Solution::is_valid(test_case);
        assert_eq!(ans, true);
    }
}
