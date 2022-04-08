impl Solution {
	// https://www.youtube.com/watch?v=gVUrDV4tZfY&ab_channel=NeetCode
    pub fn get_sum(a: i32, b: i32) -> i32 {
		let mut a = a;
		let mut b = b;

        while b != 0 {
			let tmp = (a & b) << 1;
			a = a ^ b;
			b = tmp;
		}
		a
    }
}
