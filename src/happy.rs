/**
 * Write an algorithm to determine if a number is "happy".

A happy number is a number defined by the following process: Starting with any positive integer,
replace the number by the sum of the squares of its digits, and repeat the process until the number
equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
Those numbers for which this process ends in 1 are happy numbers.
 */

impl Solution {
	pub fn is_happy(n: i32) -> bool {
		fn sum_of_sq(mut x: i32) -> i32 {
			let mut s = 0;
			while x != 0 {
				let rem = x % 10;
				s += rem * rem;
				x /= 10;
			}
			s
		}
		let mut slow = n;
		let mut fast = sum_of_sq(n);

		while fast != 1 && slow != fast {
			slow = sum_of_sq(slow);
			fast = sum_of_sq(sum_of_sq(fast));
		}

		return fast == 1;
	}
}
