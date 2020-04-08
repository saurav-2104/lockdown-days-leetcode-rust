/**
 * Given an integer array arr, count element x such that x + 1 is also in arr.
 * If there're duplicates in arr, count them seperately.
*/
use std::collections::HashSet;
impl Solution {
	pub fn count_elements(arr: Vec<i32>) -> i32 {
		let set = arr.iter().cloned().collect::<HashSet<i32>>();
		let mut c = 0;

		for n in arr {
			match set.get(&(n + 1)) {
				Some(x) => c += 1,
				None => (),
			}
		}
		c
	}
}
