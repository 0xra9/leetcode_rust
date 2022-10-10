// How to think ?
// 1- Method 1:
// First we need to check it array is empty , return 0;
// As the array is sorted , we will not need  nested for loop,
// We will create previous integer variable  starting from 0 , then  loop on the array from index 1
//( to avoid comparing element on len+1 array size),
// and compare each element with the item on the counter , if not equal increase the previous
// by one and replace element in previous with current element and return previous +1 ;
// 2- Method 2 :
// Depend on Rust vector methods :
// vec.dedup() will remove duplicate , then return vector length vec.len()
pub struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // Method 1
        // match nums.is_empty() {
        //     true => return 0,
        //     false => {
        //         let mut previous = 0;
        //         for number_index in 1..nums.len() {
        //             if nums[previous] != nums[number_index] {
        //                 previous += 1;
        //                 nums[previous] = nums[number_index];
        //             }
        //         }
        //         (previous + 1) as i32
        //     }
        // }
        // Method 2 :
        nums.dedup();
        nums.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sums() {
        let mut test_sample = vec![1, 1, 2];
        let result = Solution::remove_duplicates(&mut test_sample);
        assert_eq!(result, 2)
    }
}
