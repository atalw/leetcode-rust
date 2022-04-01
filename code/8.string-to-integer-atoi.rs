use std::i32::{MAX, MIN};

impl Solution {
	// Read in and ignore any leading whitespace.
	// Check if the next character (if not already at the end of the string) is '-' or '+'. 
	// Read this character in if it is either. This determines if the final result is negative or 
	// positive respectively. Assume the result is positive if neither is present.
	// Read in next the characters until the next non-digit character or the end of the input is 
	// reached. The rest of the string is ignored.
	// Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were 
	// read, then the integer is 0. Change the sign as necessary (from step 2).
	// If the integer is out of the 32-bit signed integer range [-2³¹, 2³¹ - 1], then clamp the 
	// integer so that it remains in the range. Specifically, integers less than -2³¹ should be 
	// clamped to -2³¹, and integers greater than 2³¹ - 1 should be clamped to 2³¹ - 1.
	// Return the integer as the final result.

	// Exploring match
    pub fn my_atoi(s: String) -> i32 {
		// let s = "00000-42a1234"; // 0
		// let s = "4193 with words";
		// let s = "-5-"; // -5
		// let s = "0-5-"; // 0
		// let s = "4.3"; // 4
		let chars: Vec<char> = s.chars().collect();
		let mut digits = "".to_string();
		let mut num: i32 = 0;
		let mut is_negative = false;
		let mult = 10;
		let mut should_be_digit_now = false;
		for c in chars.iter() {
			match c {
				c if *c == '.' => break,
				c if c.is_alphabetic() => break,
				c if *c == ' ' && !should_be_digit_now => continue,
				c if *c == ' ' && should_be_digit_now => break,
				c if *c == '-' && !should_be_digit_now => {
					is_negative = true;
					should_be_digit_now = true;
					continue
				},
				c if *c == '-' && should_be_digit_now => break,
				c if *c == '+' && !should_be_digit_now => {
					is_negative = false;
					should_be_digit_now = true;
					continue
				},
				c if *c == '+' && should_be_digit_now => break,
				c if c.is_numeric() => match c.to_digit(10) {
					Some(digit) => {
						// 123
						// 1
						// 10 + 2
						// 120 + 3
						should_be_digit_now = true;
						let om = num.overflowing_mul(mult);
						if om.1 { 
							if is_negative { return MIN } else { return MAX };
						}
						let om2 = om.0.overflowing_add(digit as i32);
						if om2.1 {
							if is_negative { return MIN } else { return MAX };
						}
						num = om2.0;
					},
					_ => continue
				}
				_ => return 0
			}
		}
		return if is_negative { -num }  else { num }
    }
}
