/**
 * Given a binary array, find the maximum length of a contiguous subarray with equal number of 0 and 1.
 */

impl Solution {
	pub fn find_max_length(nums: Vec<i32>) -> i32 {
		let mut ones = 0;
		let mut maxL: i32 = 0;
		let mut map = std::collections::HashMap::new();
		map.insert(0, 0);

		for (i, &n) in nums.iter().enumerate() {
			if n == 0 {
				ones -= 1;
			} else {
				ones += 1;
			}
			match map.get(&ones) {
				None => {
					map.insert(ones, (i + 1) as i32);
				}
				Some(&val) => maxL = std::cmp::max(maxL, (i as i32) - val + 1),
			}
		}
		maxL
	}
}
