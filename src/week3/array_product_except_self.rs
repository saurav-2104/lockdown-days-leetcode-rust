// Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].

// Example:

// Input:  [1,2,3,4]
// Output: [24,12,8,6]
// Constraint: It's guaranteed that the product of the elements of any prefix or suffix of the array (including the whole array) fits in a 32 bit integer.

// Note: Please solve it without division and in O(n).
impl Solution {
	pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
		let l = nums.len();
		let mut ans = vec![1; l];
		let mut c = 1;
		for i in 0..l {
			ans[i] = c;
			c = c * nums[i];
		}
		c = 1;
		for i in (0..l).rev() {
			ans[i] *= c;
			c *= nums[i];
		ans
	}
}
