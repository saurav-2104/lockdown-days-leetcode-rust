/**
 * Given a non-empty array of integers, every element appears twice except for one. Find that single one.
 */

impl Solution {
	pub fn single_number(nums: Vec<i32>) -> i32 {
		let mut s = nums[0];
		for i in 1..nums.len() {
			s = s ^ nums[i];
		}
		s
	}
}
