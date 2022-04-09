use std::collections::HashMap;

impl Solution {
	// did recursion (top down)
	// did recursion + memoization (top down) 
	// did recursion (bottom up)
	// did recursion + memoization (bottom up)
	// all timeout at test case n = 45
    pub fn climb_stairs(n: i32) -> i32 {
		// let mut map: HashMap<i32, i32> = HashMap::new();
		
		let mut first = 1;
		let mut second = 1;
		// let mut count = 0;

		for i in 2..=n {
			let temp = first;
			first = first + second; 
			second = temp;
		}


		first
		// Solution::climb_bottom_up(0, n, map)
	}

	fn climb_bottom_up(n: i32, target: i32, mut map: HashMap<i32, i32>) -> i32 {
		if n > target { return 0 }
		if n == target { return 1 }

		match map.get(&n) {
			Some(&val) => return val,
			None => {}
		}

		map.insert(n, Solution::climb_bottom_up(n + 1, target, map.clone()) + Solution::climb_bottom_up(n + 2, target, map.clone()));

		match map.get(&n) {
			Some(&val) => return val,
			None => unimplemented!()
		}

	}

    // pub fn climb_stairs(n: i32) -> i32 {
	//     // let mut num = n;
	//     let mut count = 0;
	//     let mut sum = 0;

	//     let mut map: HashMap<i32, i32> = HashMap::new();

	//     // match map.get(&n) {
	//     //     Some(&val) => return val,
	//     //     None => {}
	//     // }
	//     // if n <= 1 { return 1 }
	//     // if n < 0 { return 0 }

	//     map.insert(2, 2);
	//     map.insert(1, 1);
	//     map.insert(0, 1);

	//     return Solution::climb_internal(map.clone(), n);

	//     // map.insert(n - 1, Solution::climb_stairs(n - 1));
	//     // map.insert(n - 2, Solution::climb_stairs(n - 2));
		
	//     println!("{:?}", map);

	//     // count += Solution::climb_stairs(n - 2);
	//     // if num == n || num == n - 1 { return 1 }
	//     // num = n - 1;
	//     // map.insert(num, Solution::climb_stairs(num));


	//     // for i in (1..=n) {

	//     //     if n == 0 { return 1 }
	//     //     if sum == n { return 1 }
	//     //     count += Solution::climb_stairs(n - i)
	//     // }

	//     // println!("{}", sum);

	//     count
    // }

	fn climb_internal(mut map: HashMap<i32, i32>, n: i32) -> i32 {
		match map.get(&n) {
			Some(&val) => return val,
			None => {}
		}

		map.insert(n, Solution::climb_internal(map.clone(), n - 1) + Solution::climb_internal(map.clone(), n - 2));

		match map.get(&n) {
			Some(&val) => return val,
			None => panic!()
		}
	}
}
