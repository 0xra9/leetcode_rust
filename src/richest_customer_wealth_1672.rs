// https://leetcode.com/problems/richest-customer-wealth/
// How to think?
// method 1 :
// Loop on each vector and push its sum into result vector and then return the max of the result vector
// method 2 :
// depends on iterator method map : which do operation on each element, in our case it will return sum of each vector and then use reduce to get max of the output
pub struct Solution {}
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        // method 1
//        let mut result:Vec<i32> = Vec::new();
//        for account in &accounts{
//            result.push(account.iter().sum());
//        }
//        println!("{:?}",result);
//
//        result.into_iter().reduce(i32::max).unwrap()
        //method 2
        accounts.iter().map(|acc|acc.iter().sum()).reduce(i32::max).unwrap()


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_wealth() {
        let test_case: Vec<Vec<i32>> = vec![vec![2,8,7],vec![7,1,3],vec![1,9,5]];
        let ans = Solution::maximum_wealth(test_case);
        assert_eq!(ans, 17);
    }
}
