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
	// A BF CE D E
    pub fn convert(s: String, num_rows: i32) -> String {
		let s = "ABCDEF".to_string();
		let num_rows = 5;
		let mut res = "".to_string();
		let mut count = if num_rows % 2 == 0 { num_rows + 1 } else { num_rows };
		let chars: Vec<char> = s.chars().collect();
		let mut odd_vals: Vec<i32> = Vec::new();
		for i in 0..num_rows {
			if i % 2 != 0 {
				odd_vals.push(i);	
			}
		}
		// println!("{:?}", odd_vals);
		if num_rows == 1 { return s }
		if num_rows == s.len() as i32 { return s }
		count = if num_rows == 2 { 1 } else { count };
		for i in 0..num_rows {
			match i {
				0 => { // first index
					let mut j = i;
					while j < s.len() as i32 {
						res.push(chars[j as usize]);
						j = j+count+1;
					}
				},
				i if i % (num_rows-1) == 0 => { // last index
					let mut j = i;
					while j < s.len() as i32 {
						res.push(chars[j as usize]);
						j = j+count+1;
					}
				},
				i if (i+1) % 2 == 0 => {
					let mut j = i;
					let mut index = odd_vals.len()-1;
					while j < s.len() as i32 {
						res.push(chars[j as usize]);
						j = j+odd_vals[index]+1;
						index += 1;
						index = if index % odd_vals.len() == 0 { 0 } else { index }; // loop index
					}
				},
				_ => {
					let mut j = i;
					let mut index = 0;
					while j < s.len() as i32 {
						res.push(chars[j as usize]);
						j = j+odd_vals[index]+1;
						index += 1;
						index = if index % odd_vals.len() == 0 { 0 } else { index }; // loop index
					}
				}
			}
		}
		res
    }
}
