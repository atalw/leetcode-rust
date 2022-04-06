use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
		// let digits = "232";

		let map: HashMap<char, &str> = Solution::create_mapping();
		let chars: Vec<char> = digits.chars().collect();
		let mut res: Vec<String> = Vec::new();

		// bad solution
		for c in chars {
			match map.get(&c) {
				Some(&letters) => {
					let mut strings: Vec<String> = Vec::new();
					for cc in letters.chars() {
						if res.is_empty() { strings.push(cc.to_string()) }
						else {
							for r in res.iter() {
								strings.push(r.to_string() + &cc.to_string());
							}
						}
					}
					res = strings;
				}
				None => {}
			}
		}

		res
    }

	fn create_mapping() -> HashMap<char, &'static str> {
		let mut map: HashMap<char, &str> = HashMap::new();

		map.insert('2', "abc");
		map.insert('3', "def");
		map.insert('4', "ghi");
		map.insert('5', "jkl");
		map.insert('6', "mno");
		map.insert('7', "pqrs");
		map.insert('8', "tuv");
		map.insert('9', "wxyz");

		map
	}
}
