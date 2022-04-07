impl Solution {
    pub fn is_valid(s: String) -> bool {
		// let s = "(){[]}[]";
		// let s = "({{}})";
		// let s = "({{}})]";
		let mut stack: Vec<char> = Vec::new();
		let chars: Vec<char> = s.chars().collect();
		let open = vec!['(', '{', '['];
		let close = vec![')', '}', ']'];


		for c in chars {
			match open.contains(&c) {
				true => { stack.push(c); continue },
				false => {}
			}
			match close.iter().position(|&x| x == c) {
				Some(idx) => {
					match stack.last() {
						Some(&st) if open[idx] == st => { stack.pop(); },
						_ => return false
					}
				},
				_ => return false
			}
		}

		if stack.is_empty() { true } else { false } 
    }
}
