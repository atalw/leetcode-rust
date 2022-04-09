use std::collections::HashMap;

impl Solution {
	// top down recursion exceeds time
	// add memoization
	// recursive memoization also exceeds time limit
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
		let mut map: HashMap<i32, i32> = HashMap::new();
        
		// println!("{:?}----------", amount);
		Solution::coin_change_internal(map, coins, amount, 0)
    }

	fn coin_change_internal(mut map: HashMap<i32, i32>, coins: Vec<i32>, val: i32, count: i32) -> i32 {
		let mut count = count;
		if val == 0 { return count }
		if val < 0 { return -1 }

		match map.get(&val) {
			Some(&m) => return m,
			None => {}
		}

		for coin in &coins {
			// update count each recursion and return count when val == 0
			let res = Solution::coin_change_internal(map.clone(), coins.clone(), val - coin, count + 1);
			if res > 0 { map.insert(val, res); }
		}

		// println!("{:?}", arr);

		match map.get(&val) {
			Some(&m) => m,
			None => -1
		}
	}
}
