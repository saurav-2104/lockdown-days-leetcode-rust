// You are given a string s containing lowercase English letters, and a matrix shift, where shift[i] = [direction, amount]:

// direction can be 0 (for left shift) or 1 (for right shift).
// amount is the amount by which string s is to be shifted.
// A left shift by 1 means remove the first character of s and append it to the end.
// Similarly, a right shift by 1 means remove the last character of s and add it to the beginning.
// Return the final string after all operations.

impl Solution {
	pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
		let mut count = 0;
		let len = s.chars().count() as i32;
		for i in shift {
			if i[0] == 0 {
				count -= i[1]
			} else {
				count += i[1]
			}
		}
		let (s1, s2) = s.split_at(((len - (count % len)) % len) as usize);
		format!("{}{}", s2, s1)
	}
}
