use std::collections::HashMap;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
		let table: HashMap<i32, String> = Solution::setup_roman_table();
		let mut roman: String = "".to_string();

		let mut i = 0;
		let mut num = num;
		let mut rem = 0;

		while num > 0 {
			rem = num % 10 * 10_i32.pow(i);
			num /= 10;
			i += 1;

			match table.get(&rem) {
				Some(numeral) => { // If direct match with table
					roman.insert_str(0, numeral);
					continue;
				}
				None => {
					let val = rem.clone();
					let mut less = table.clone();
					less.retain(|&x, _| x < val);
					let mut keys: Vec<&i32> = less.keys().collect();
					keys.sort();
					let mut numeral = "".to_string();
					while rem > 0 {
						let mut largest = keys.iter().max_by_key(|&&x| x <= &rem).unwrap();
						numeral.push_str(less.get(largest).unwrap());
						rem = rem - *largest;
					}
					roman.insert_str(0, &numeral);
				}
			}
		}

		roman
    }

	fn setup_roman_table() -> HashMap<i32, String> {
		let mut table: HashMap<i32, String> = HashMap::new();
		table.insert(1, "I".to_string());
		table.insert(4, "IV".to_string());
		table.insert(5, "V".to_string());
		table.insert(9, "IX".to_string());
		table.insert(10, "X".to_string());
		table.insert(40, "XL".to_string());
		table.insert(50, "L".to_string());
		table.insert(90, "XC".to_string());
		table.insert(100, "C".to_string());
		table.insert(400, "CD".to_string());
		table.insert(500, "D".to_string());
		table.insert(900, "CM".to_string());
		table.insert(1000, "M".to_string());
		table
	}
}
