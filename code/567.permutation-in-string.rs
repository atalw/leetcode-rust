use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
		// long => ngol
		// ngoler
		// rwvrk
		// abwrvrk, abwvrrk, arwvrkb

		let mut map: HashMap<char, i32> = HashMap::new();
		let mut m: HashMap<char, i32> = HashMap::new();

		for c in s1.chars() {
			map.entry(c).or_insert(0);
			map.insert(c, 1 + map[&c]);
		}

		let mut start = 0;
		let mut end = s1.len() - 1;

		while end < s2.len() {
			m.clear();
			for c in s2.get(start..=end).unwrap().chars() {
				m.entry(c).or_insert(0);
				m.insert(c, 1 + m[&c]);
			}

			if map == m { return true }

			start += 1;
			end += 1;
		}

		false
    }

	// not needed lmao
	// fn is_permutation(s1: &str, s2: &str) -> bool {
	//     if s2 == "" { return false }
		
	//     let mut string = s2.to_string();
	//     for i in 0..s1.len() {
	//         let c = s1.chars().nth(i).unwrap();
	//         if !string.contains(c) { return false }

	//         // remove from s2
	//         let index = string.find(c).unwrap();
	//         string.remove(index);
	//     }

	//     string.is_empty()
	// }


	// ----Brute force solution----
    // pub fn check_inclusion(s1: String, s2: String) -> bool {
	//     Solution::check_inclusion_internal(&s1, &s2, "")
    // }

	// fn check_inclusion_internal(s1: &str, s2: &str, permutation: &str) -> bool {
	//     let mut given_string = s1.to_string();

	//     if s1.len() == 0 {
	//         return s2.contains(permutation)
	//     }

	//     for i in 0..s1.len() {
	//         // choose
	//         let c = given_string.remove(i);
	//         let mut chosen_string = permutation.to_owned() + &c.to_string();

	//         // explore
	//         if Solution::check_inclusion_internal(&given_string, s2, &chosen_string) { return true }

	//         // unchoose
	//         given_string.insert(i, c);
	//         chosen_string.remove(chosen_string.len() - 1);
	//     }

	//     false
	// }
}
