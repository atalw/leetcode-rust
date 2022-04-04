use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
		let table: HashMap<String, i32> = Solution::setup_roman_table();

		let mut chars: Vec<char> = s.chars().collect();
		let mut number = 0;

		let mut idx = 0;

		while idx < chars.len() {
			let c = chars[idx];
			let next_idx = idx + 1;

			if next_idx < s.len() {
				let combined = c.to_string() + &chars[next_idx].to_string();
				if let Some(numeral) = table.get(&combined) {
					number += numeral;
					idx = next_idx + 1;
				} else {
					if let Some(numeral) = table.get(&c.to_string()) {
						number += numeral;
						idx += 1;
					}
				}
			} else {
				if let Some(numeral) = table.get(&c.to_string()) {
					number += numeral;
					idx += 1;
				}
			}
		}

		return number
    }


	fn setup_roman_table() -> HashMap<String, i32> {
		let mut table: HashMap<String, i32> = HashMap::new();
		table.insert("I".to_string(), 1);
		table.insert("IV".to_string(), 4);
		table.insert("V".to_string(), 5);
		table.insert("IX".to_string(), 9);
		table.insert("X".to_string(), 10);
		table.insert("XL".to_string(), 40);
		table.insert("L".to_string(), 50);
		table.insert("XC".to_string(), 90);
		table.insert("C".to_string(), 100);
		table.insert("CD".to_string(), 400);
		table.insert("D".to_string(), 500);
		table.insert("CM".to_string(), 900);
		table.insert("M".to_string(), 1000);
		table
	}

}
