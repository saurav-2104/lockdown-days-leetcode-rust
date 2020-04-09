/**
	Given two strings S and T, return if they are equal when both are typed into empty text editors.
	# means a backspace character.

	Example 1:

	Input: S = "ab#c", T = "ad#c"
	Output: true
	Explanation: Both S and T become "ac".
*/

impl Solution {
	pub fn backspace_compare(s: String, t: String) -> bool {
		fn reduce_str(x: String) -> Vec<char> {
			let mut res = vec![];
			for c in x.chars() {
				if c == '#' {
					res.pop();
				} else {
					res.push(c);
				}
			}
			res
		}

		// Space is O(N)
		reduce_str(s) == reduce_str(t)
	}

	pub fn backspace_compare(s: String, t: String) -> bool {
		// Space - O(1)
		let st: Vec<char> = s.chars().collect::<Vec<char>>();
		let tt: Vec<char> = t.chars().collect::<Vec<char>>();
		let mut sb = 0;
		let mut tb = 0;

		let mut i: i32 = st.len() as i32 - 1;
		let mut j: i32 = tt.len() as i32 - 1;

		loop {
			while i >= 0 && (sb > 0 || (st[i as usize] == '#')) {
				if st[i as usize] == '#' {
					sb += 1;
				} else {
					sb -= 1;
				}
				i -= 1;
			}
			while j >= 0 && (tb > 0 || (tt[j as usize] == '#')) {
				if tt[j as usize] == '#' {
					tb += 1;
				} else {
					tb -= 1;
				}
				j -= 1;
			}
			if !(i >= 0 && j >= 0 && st[i as usize] == tt[j as usize]) {
				return i == -1 && j == -1;
			}
			i -= 1;
			j -= 1;
		}

		true
	}
}
