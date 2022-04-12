use std::collections::HashMap;

impl Solution {
	// https://youtu.be/Sx9NNgInc3A
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {

		let mut dp: HashMap<usize, bool> = HashMap::new();
		dp.insert(s.len(), true);

		for i in (0..s.len()).rev() {
			for word in &word_dict {
				let end = i + word.len();
				match (s.get(i..end), dp.get(&end)) {
					(Some(a), Some(&b)) if a == word => { dp.insert(i, b); },
					_ => {}
					
				}
				match dp.get(&i) {
					Some(&val) if val => break,
					_ => {}
				}
			}
		}

		match dp.get(&(0 as usize)) {
			Some(&val) => val,
			None => false
		}
	}
}

