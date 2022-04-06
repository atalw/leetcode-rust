impl Solution {
    pub fn to_hex(num: i32) -> String {
		// let num: i32 = -1322;
		// let num: i32 = 1322;
		// let num: i32 = -2147483648;
		// println!("MAX: {}, MIN: {}", std::i32::MAX, std::i32::MIN);
		let mut hex = "".to_string();

		let map = "0123456789abcdef";
		let set: Vec<char> = map.chars().collect();

		match num {
			mut num if num > 0 => {
				while num > 15 {
					let rem = num % 16;
					num = num / 16;
					hex.insert(0, set[rem as usize]);
				}
				hex.insert(0, set[num as usize]);
			},
			num if num == 0 => {
				hex.push_str("0");
			},
			num if num < 0 => {
				// https://stackoverflow.com/a/58030505
				let mut positive = num.abs() as u32;
				let mut bits = "".to_string();

				let mut dec = num.abs() as u32;
				// println!("positive: {}, {}", positive, num.abs());

				// convert to binary
				// not needed: leaving this in for explanation
				// plus this logic fails when num = i32::MIN because the abs is bigger than i32MAX
				while positive > 0 {
					if positive % 2 == 0 { bits.push_str("0") }
					else { bits.push_str("1") }
					positive /= 2
				}

				bits = bits.chars().rev().collect();

				let mut _dec = match i32::from_str_radix(&bits, 2) {
					Ok(val) => val,
					// Err(e) => panic!("{}, bits: {}", e, bits) // panics when num = MAX
					Err(e) => 0
				};

				// assert_eq!(dec, num.abs());

				// negate binary
				dec = !dec;
				// add one
				dec = dec + 0b1;

				// println!("positive: {}, {}", dec, !_dec+0b1);

				// convert to string with appropriate 0 padding
				let binary_string = format!("{:032b}", dec);
				
				let mut index = 0;
				let mut sum = 0;
				for c in binary_string.chars().rev() {
					match c.to_digit(2) {
						Some(n) => sum += n * 2_u32.pow(index),
						None => panic!()
					}
					
					if index == 3 {
						hex.insert(0, set[sum as usize]);
						sum = 0;
						index = 0
					} else {
						index += 1;
					}
				}

				// println!("{}", binary_string);
				// println!("{:b} {:b}", binary, res);

			},
			_ => panic!()
		}

		hex
    }
}
