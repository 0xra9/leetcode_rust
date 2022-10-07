use std::ops::Index;

// How to think ?
// 1- Method 1:
// The most obvious thinking is to have 2 nested for loops through the input vector,
// and compare the sum with the target , this will produce vector with the results , but we will need
// to remove the duplicates from the results vector
// We can use hashmap to remove the duplicate from the start and make sure that the results are
// unique , then convert this hashmap into result vector
// This method will solve the problem but needs optimization
// 2- Method 2:
// For optimization , we will depend on math :
// if we subtract the target from one element of the array , and the result of subtraction  is in the array ,
// that means that the 2 elements are the answer
pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Method 1
        // use std::collections::HashMap;
        // let mut ans_hash_map = HashMap::new();
        // for (i, &item1) in nums.iter().enumerate() {
        //     for (j, &item2) in nums.iter().enumerate() {
        //         // i and j must be not equal to prevent adding the same number twice
        //         if &item1 + &item2 == target && i != j {
        //             ans_hash_map.insert(i as i32, j as i32);
        //         }
        //     }
        // }
        // // collect the values from hash map into the result vector
        // let mut ans = ans_hash_map.values().cloned().collect::<Vec<i32>>();
        // ans
        //
        // 2- Method 2 :
        use std::collections::HashMap;
        let mut hash_map_ans = HashMap::new();
        for (pos, item) in nums.iter().enumerate() {
            // if the hashmap contain the result it will return it , otherwise result will be inserted
            match hash_map_ans.get(&(target - item)) {
                Some(found) => return vec![*found as i32, pos as i32],
                None => hash_map_ans.insert(*item, pos),
            };
        }
        // if no result found return empty vector
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sums() {
        let test_sample = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(test_sample, target);
        println!("Result: {:?}", result);
        let right_ans = vec![2, 1];
        assert_eq!(result, right_ans);
    }
}
