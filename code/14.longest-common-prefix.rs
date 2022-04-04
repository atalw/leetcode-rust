impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
		// [apple, beetle, little] = ""
		// [buddha, buddy, budding] = budd
		// [lamp, ramp, map] = ""
		// [map, map, map] = map
		

		match strs.is_empty() {
			true => "".to_string(),
			_ => {
				strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
					acc
						.chars()
						.zip(x.chars())
						.take_while(|(x, y)| x == y)
						.map(|(x, _)| x)
						.collect()
				})
			}
		}


		// let mut prefix = "".to_string();
		// let mut idx = 0;

		// while idx < strs[0].len() {
		//     let letter = strs[0].chars().nth(idx).unwrap();
		//     for s in strs.iter().skip(1) {
		//         match s.chars().nth(idx) {
		//             Some(c) if c == letter => { continue },
		//             _ => return prefix
		//         }
		//     }
		//     prefix.push(letter);
		//     idx += 1;
		// }


		// prefix
    }
}
