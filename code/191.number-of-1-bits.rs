impl Solution {
	// https://www.youtube.com/watch?v=5Km3utixwZs&ab_channel=NeetCode
    pub fn hammingWeight (n: u32) -> i32 {
		let mut count = 0;
		let mut n = n;

		while n != 0 {
			if n % 2 != 0 { count += 1 }
			n = n >> 1;
		}

		count
    }
}
