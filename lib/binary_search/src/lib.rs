/*
Given a sorted (in ascending order) integer array nums of n elements and a target value, write a function to search target in nums. If target exists, then return its index, otherwise return -1.


Example 1:

Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4

Example 2:

Input: nums = [-1,0,3,5,9,12], target = 2
Output: -1
Explanation: 2 does not exist in nums so return -1

 

Note:

    You may assume that all elements in nums are unique.
    n will be in the range [1, 10000].
    The value of each element in nums will be in the range [-9999, 9999].


*/

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // standard binary search impl
    // use std::cmp;
    // let mut low = nums[0];
    // let mut high = cmp::max(low, nums[nums.len() - 1] + 1);
    // let mut mid: i32 = 0;

    // while low < high {
    //     mid = (low + high) / 2;
    //     println!("{}", mid);
    //     if target <= nums[mid as usize] {
    //         high = mid;
    //     } else {
    //         low = mid + 1
    //     }
    // }
    match nums.iter().position(|&v| v == target) {
        Some(x) => return x as i32,
        None => return -1,
    }
    // if high == target{
    //     return nums.iter().position(|&v| v == target).unwrap() as i32;
    // } else {
    //     return -1
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-1,0,3,5,9,12];
        let target = 9;
        let result = 4;
        assert_eq!(search(nums, target), result);
    }
}