impl Solution {
	pub fn length_of_longest_substring(s: String) -> i32 {
		// let s = " ";
		// let s = "";
		// let s = "aa";
		let mut max_len = 0;

		let mut substring: String = "".to_string();
		let mut st: Vec<char> = s.chars().collect();
		let mut index = 0;

		while index < s.len() {
			let c = st[index];
			substring.push(c);

			let mut subindex = index + 1;

			while subindex < s.len() {
				let c = st[subindex];
				if !substring.contains(c) {
					substring.push(c);
					subindex += 1;
				} else {
					break;
				}
			}

			if substring.len() > max_len { max_len = substring.len() };
			substring = "".to_string();

			index += 1;
		}
		return max_len as i32
	}
}
