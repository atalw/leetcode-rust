impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
		// [apple, beetle, little] = ""
		// [buddha, buddy, budding] = budd
		// [lamp, ramp, map] = ""
		// [map, map, map] = map
		
		let mut prefix = "".to_string();
		let mut idx = 0;

		while idx < strs[0].len() {
			let letter = strs[0].chars().nth(idx).unwrap();
			for s in strs.iter().skip(1) {
				match s.chars().nth(idx) {
					Some(c) if c == letter => { continue },
					_ => return prefix
				}
			}
			prefix.push(letter);
			idx += 1;
		}


		prefix
    }
}
