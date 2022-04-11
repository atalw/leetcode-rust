use std::collections::HashMap;

impl Solution {
	// help: https://youtu.be/Ua0GhsJSlWM
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
		let mut dp: HashMap<(usize, usize), i32> = HashMap::new();

		let mut max_len = if text1.len() > text2.len() { text1.len() } else { text2.len() };

		for i in (0..text1.len()).rev() {
			for j in (0..text2.len()).rev() {
				if text1.chars().nth(i) != text2.chars().nth(j) {
					match (dp.get(&(i+1, j)), dp.get(&(i, j+1))) {
						(Some(&a), Some(&b)) => { dp.insert((i, j), a.max(b)); },
						(Some(&a), None) => { dp.insert((i, j), a.max(0)); },
						(None, Some(&b)) => { dp.insert((i, j), 0.max(b)); },
						(None, None) => { dp.insert((i, j), 0.max(0)); },
					}
				} else {
					match (dp.get(&(i+1, j+1))) {
						Some(&item) => { dp.insert((i, j), 1 + item); },
						None => { dp.insert((i, j), 1); },
					}
				}
			}
		}

		match dp.iter().max_by_key(|x| x.1) {
			Some((&key, &val)) => val,
			None => 0
		}
    }
}
