impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
		let nums: Vec<i32> = (0..=n).collect();
		let mut res: Vec<i32> = Vec::new();
		let mut i = 0;

		for mut num in nums {
			res.push(0);

			while num != 0 {
				if num % 2 != 0 { res[i] += 1 }
				num = num >> 1;
			}

			i += 1;
		}

		res
    }
}
