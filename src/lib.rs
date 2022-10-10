extern crate core;


mod maximum_nesting_depth_of_the_parentheses;
mod remove_duplicates_from_sorted_array_26;
mod roman_to_integer_13;
mod valid_parentheses;
mod two_sum_1;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
