// https://leetcode.com/problems/number-of-good-pairs/
// How to think?
// The classical way is to loop through the vector in nested for loop and check the condition
// nums[i] == nums[j] and i < j , and if found , push the i to vector and then return vector length.
pub struct Solution {}
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut ans = Vec::new();
        for i in 0..nums.len(){
            for j in 0..nums.len(){
                if nums[i] == nums[j] && i <j {
                    ans.push(i);
                }
            }
        }
        ans.len()  as i32

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_identical_pairs() {
        let test_case = vec![1,2,3,1,1,3];
        let ans = Solution::num_identical_pairs(test_case);
        println!("ans : {}",ans);
        assert_eq!(ans, 4);
    }
}
