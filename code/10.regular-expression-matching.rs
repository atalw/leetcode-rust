impl Solution {
	// Incomplete
    pub fn is_match(s: String, p: String) -> bool {
		// let s = "mississippi";
		// let p = "mis*is*ip*.";

		// let s = "ab";
		// let p = ".*c"; // false

		// let s = "aa";
		// let p = "a*";

		// let s = "aaaa";
		// let p = "a*a";
		// let p = "a*aa";

		// let s = "aab";
		// let p = "a*b";

		// let s = "aaab";
		// let p = "a*ab";
		// let p = "a*b";
		// let p = "aaab";
		// let p = "aab"; // false

		// let s = "aaaaaaaab";
		// let p = "a*aab";

		// let s = "cab";
		// let p = "c*a*b";

		// let s = "aaa";
		// let p = "ab*ac*a";
		// let p = "ab*aac*";
		// let p = "aaac*";
		// let p = "aaac*d*e*";
		// let p = "ab*a*c*a";
		// let p = "ab*a*c*d*a";
		// let p = "ab*a*c*d*b"; // fales

		// let s = "aa";
		// let p = "a"; // false

		// let s = "aa";
		// let p = "aaa"; // false

		// let s = "aaba";
		// let p = "ab*a*c*a"; // false

		let s = "a";
		let p = "ab*a"; // false

		// let s = "aab";
		// let p = "aab*b"; // true 

		let s_chars: Vec<char> = s.chars().collect();
		let p_chars: Vec<char> = p.chars().collect();

		let mut s_index = 0;
		let mut p_index = 0;

		let mut asterisk_letter: Option<char> = None;
		let mut is_period = false;

		while s_index < s_chars.len() && p_index < p_chars.len() {
			let sc = s_chars[s_index];
			let pc = p_chars[p_index];

			match pc {
				pc if p_index + 1 < p_chars.len() && p_chars[p_index + 1] == '*' => { // match a*
					asterisk_letter = Some(pc);
					p_index += 1;
				},
				'*' => if s_index > 0 { asterisk_letter = Some(p_chars[p_index-1]); } else { return false },
				'.' => { is_period = true },
				_ => { asterisk_letter = None; is_period = false; }
			}

			match asterisk_letter {
				Some(letter) if sc == letter || letter == '.' => {
					s_index += 1;
					if s_index == s_chars.len() {
						// catch up p_index incase next letter of * is the same as asterisk_letter
						p_index += 1;
						while p_index < p_chars.len() && s_chars[s_index - 1] == p_chars[p_index] {
							// if p_index == p_chars.len() { break }
							p_index += 1;
						}
					}
					asterisk_letter = None;
					continue
				},
				// Some(letter) if letter == '*' => { return false }, // handle ** case
				Some(letter) => { // no match
					p_index += 1;
					if s_index == s_chars.len() {
						// catch up p_index incase next letter of * is the same as asterisk_letter
						while s_chars[s_index - 1] == p_chars[p_index] {
							if p_index == p_chars.len() - 1 { break }
							p_index += 1;
						}
					}
					asterisk_letter = None;
					continue
				}
				None => {}
			}

			println!("before{} : {}", s_index, p_index);
			if sc == pc || is_period {
				s_index += 1;
				p_index += 1;
				continue
			} else {
				println!("here{} : {}", s_index, p_index);
				return false;
			}

		}
		println!("after{} : {}", s_index, p_index);
		// Check if fully match
        if s_index == s_chars.len() { 
			match p_index {
				// check if pattern's last char is '*'
				idx if idx == p_chars.len() - 1 && p_chars[idx] == '*' => true, // last char is *
				// idx if idx == p_chars.len() - 2 && p_chars[p_chars.len() - 1] == '*' => true, // s = "aaa", p = "aaac*"
				idx if idx == p_chars.len() => true,
				_ => {
					if Solution::is_asterisk_pair(s_chars, p_chars, p_index) { true }
					else { false }
				}
			}
		} else {
			false
		}
    }

	// aaa
	// let p = aaac*d*e*;
	// let p = "ab*a*c*d*a";
	fn is_asterisk_pair(s_chars: Vec<char>, p_chars: Vec<char>, index: usize) -> bool {
		let mut has_asterisk = false;
		let mut index = index;
		while index < p_chars.len() - 1 {
			let letter = p_chars[index];
			let asterisk = p_chars[index + 1];

			println!("expect: {}", index);

			if asterisk == '*' { index += 2; has_asterisk = true } else { return false }
		}

		if index == p_chars.len() { return true } 
		else { 
			// s = aaa, p = "ab*a*c*d*a";
			if index == p_chars.len() - 1 && has_asterisk {
				let letter = p_chars[index];
				if &letter == s_chars.last().unwrap() { return true } else { return false }
			} else { 
				return false 
			}
		}
	}
}
