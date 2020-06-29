/*
Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You may assume no duplicates in the array.

Example 1:

Input: [1,3,5,6], 5
Output: 2

Example 2:

Input: [1,3,5,6], 2
Output: 1

Example 3:

Input: [1,3,5,6], 7
Output: 4

Example 4:

Input: [1,3,5,6], 0
Output: 0

*/

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // Binary search, since its already sorted
    use std::cmp;
    let mut low = 0;
    let mut high = cmp::max(low, nums.len() -1);
    let mut mid = 0;

    while low < high {
        mid = (low + high) / 2;
        if target <= nums[mid] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    // match the binary search result against the target
    if nums[high] >= target {
        return high as i32
    } else {
        return high as i32 + 1;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1,3,5,6];
        let target = 5;
        let answer = 2;
        assert_eq!(search_insert(nums, target), answer);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1,3,5,6];
        let target = 2;
        let answer = 1;
        assert_eq!(search_insert(nums, target), answer);
    }

    #[test]
    fn it_works3() {
        let nums = vec![1,3,5,6];
        let target = 7;
        let answer = 4;
        assert_eq!(search_insert(nums, target), answer);
    }
    #[test]
    fn it_works4() {
        let nums = vec![1,3,5,6];
        let target = 0;
        let answer = 0;
        assert_eq!(search_insert(nums, target), answer);
    }

    #[test]
    fn it_works5() {
        let nums = vec![1];
        let target = 2;
        let answer = 1;
        assert_eq!(search_insert(nums, target), answer);
    }
}
