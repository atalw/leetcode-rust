use std::collections::HashMap;

impl Solution {
	// https://youtu.be/IlEsdxuD4lY
    pub fn unique_paths(m: i32, n: i32) -> i32 {

		let mut map: HashMap<(i32, i32), i32> = HashMap::new();

		let mut row = m;
		let mut col = n - 1;

		map.insert((row, col + 1), 1);

		while row > 0 {
			if col == 0 && row >= 1 {
				row -= 1;
				col = n;
			} else {
				match (map.get(&(row + 1, col)), map.get(&(row, col + 1))) {
					(Some(&x), Some(&y)) => { map.insert((row, col), x + y); },
					(None, Some(&y)) => { map.insert((row, col), y); },
					(Some(&x), None) => { map.insert((row, col), x); },
					(None, None) => panic!(),
					_ => {}
				}
				col -= 1;
			}
		}

		match map.get(&(1, 1)) {
			Some(&val) => val,
			None => unimplemented!()
		}
    }

	// time limit exceeded without memoization
	// TLE with memoization too
	// Time: O (2 ^ m*n)
    // pub fn unique_paths(m: i32, n: i32) -> i32 {
	//     let map: HashMap<(i32, i32), i32> = HashMap::new();
	//     Solution::unique_internal(map, m, n)
    // }

	// fn unique_internal(mut map: HashMap<(i32, i32), i32>, m: i32, n: i32) -> i32 {
	//     if m == 1 && n == 1 { return 1 }

	//     match map.get(&(m,n)) {
	//         Some(&val) => return val,
	//         None => {}
	//     }

	//     let mut count = 0;
	//     if m == 1 && n > 1 {
	//         map.insert((m, n), Solution::unique_internal(map.clone(), m, n - 1));
	//     } else if n == 1 && m > 1 {
	//         map.insert((m, n), Solution::unique_internal(map.clone(), m - 1, n));
	//     } else {
	//         map.insert((m, n), Solution::unique_internal(map.clone(), m - 1, n) + Solution::unique_internal(map.clone(), m, n - 1));
	//     }
		
	//     match map.get(&(m,n)) {
	//         Some(&val) => return val,
	//         None => panic!()
	//     }
	// }
}
