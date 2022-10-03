// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
pub struct Solution {}
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut balance =0;
        balance.max()
        let mut max_depth =0;
        for char in s.chars(){
            match char {
                '(' => balance +=1,
                ')' => balance -= 1,
                _ => continue,
            }
            if balance > max_depth{
                max_depth= balance
            }
        }

        max_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_nested_vps_depth() {
        let test_case = String::from("(1+(2*3)+((8)/4))+1");
        let ans = Solution::max_depth(test_case);
        assert_eq!(ans, 3);
    }
}
