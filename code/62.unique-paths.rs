use std::collections::HashMap;

impl Solution {
	// time limit exceeded without memoization
	// TLE with memoization too
    pub fn unique_paths(m: i32, n: i32) -> i32 {
		let map: HashMap<(i32, i32), i32> = HashMap::new();
		Solution::unique_internal(map, m, n)
    }

	fn unique_internal(mut map: HashMap<(i32, i32), i32>, m: i32, n: i32) -> i32 {
		if m == 1 && n == 1 { return 1 }

		match map.get(&(m,n)) {
			Some(&val) => return val,
			None => {}
		}

		let mut count = 0;
		if m == 1 && n > 1 {
			map.insert((m, n), Solution::unique_internal(map.clone(), m, n - 1));
		} else if n == 1 && m > 1 {
			map.insert((m, n), Solution::unique_internal(map.clone(), m - 1, n));
		} else {
			map.insert((m, n), Solution::unique_internal(map.clone(), m - 1, n) + Solution::unique_internal(map.clone(), m, n - 1));
		}
		
		match map.get(&(m,n)) {
			Some(&val) => return val,
			None => panic!()
		}
	}
}
