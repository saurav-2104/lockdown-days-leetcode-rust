// Given a string containing only three types of characters: '(', ')' and '*', write a function to check whether this string is valid. We define the validity of a string by these rules:

// Any left parenthesis '(' must have a corresponding right parenthesis ')'.
// Any right parenthesis ')' must have a corresponding left parenthesis '('.
// Left parenthesis '(' must go before the corresponding right parenthesis ')'.
// '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string.
// An empty string is also valid.

impl Solution {
	pub fn check_valid_string(s: String) -> bool {
		let mut c = 0;
		for i in s.chars() {
			if i == '(' || i == '*' {
				c += 1;
			} else {
				c -= 1;
			}
			if c < 0 {
				return false;
			}
		}
		let mut c = 0;
		for i in s.chars().rev() {
			if i == ')' || i == '*' {
				c += 1;
			} else {
				c -= 1;
			}
			if c < 0 {
				return false;
			}
		}
		true
	}
}
