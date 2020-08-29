// /*
// Given an array of integers, return indices of the two numbers such that they add up to a specific target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// Example:

// Given nums = [2, 7, 11, 15], target = 9,

// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].
// */
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
	for (idx1, &v) in nums.iter().enumerate() {
		let offset = idx1 + 1;
		for (idx2, &v2) in nums[offset..].iter().enumerate() {
			if v + v2 == target {
				return vec![idx1 as i32, (idx2 + offset) as i32];
			}
		}
	}

	vec![]
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn can_find_sums() {
		let test_vec = vec![2, 7, 11, 15];
		let test_target = 9;
		let expected_result: Vec<i32> = vec![0, 1];
		assert_eq!(expected_result, two_sum(test_vec, test_target));
	}

	#[test]
	fn can_find_sums_2() {
		let test_vec = vec![3,2,4];
		let test_target = 6;
		let expected_result: Vec<i32> = vec![1, 2];
		assert_eq!(expected_result, two_sum(test_vec, test_target));
	}
}