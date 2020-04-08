/**
 * Given an array nums, write a function to move all 0's to the end of it while maintaining the
 * relative order of the non-zero elements.
 *
 * Example:
 * Input: [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 *
 */

impl Solution {
	pub fn move_zeroes(nums: &mut Vec<i32>) {
		let mut s = 0;
		for i in 0..nums.len() {
			if nums[i] != 0 {
				nums.swap(i, s);
				s += 1;
			}
		}
	}
}
