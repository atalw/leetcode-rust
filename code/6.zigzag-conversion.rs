use std::collections::HashMap;

impl Solution {
	// PAYPAL 3
	// PA APL Y
	// 3  1   3 (GAP)
	// PAYPALISHIRING 3
	// PAHN APLSIIG YIR
	// 3    1       3 (GAP)
	// PAYPALISHIRING 4
	// PIN ALSIG YAHR PI
	// 5   3,1   1,3  5 (GAP)
	// ABC 2
	// AC B
	// ABCDEF 5
	// A B C DF E
	// ABCDEFGHIJ 5
	// AJ BI CH DF E
    pub fn convert(s: String, num_rows: i32) -> String {
		let mut strings = vec![String::new(); num_rows as usize];
		let mut i = 0;
		let mut down = true;

		if num_rows < 2 {
			return s
		}

		for c in s.chars() {
			strings[i].push(c);
			i = if down { i + 1 } else { i - 1 };
			down = down == { i > 0 && i < num_rows as usize - 1 };
		}

		strings.concat()
    }
}
