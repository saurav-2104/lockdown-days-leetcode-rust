/**
 * Given an integer array nums, find the contiguous subarray (containing at least one number)
 * which has the largest sum and return its sum.
 *
 * Example:
 * Input: [-2,1,-3,4,-1,2,1,-5,4],
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 */

impl Solution {
	pub fn max_sub_array(nums: Vec<i32>) -> i32 {
		let (mut c, mut m) = (nums[0], nums[0]);
		for i in 1..nums.len() {
			c = std::cmp::max(c + nums[i], nums[i]);
			m = std::cmp::max(m, c);
		}
		m
	}
}
