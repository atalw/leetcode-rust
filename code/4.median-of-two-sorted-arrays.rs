impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        // {1, 2}
		// {3, 4, 5, 6}
		// 3.5

		let mut n: Vec<i32> = Vec::new();
		n.append(&mut nums1);
		n.append(&mut nums2);
		n.sort();

		let total_count = n.len();
		match total_count {
			even if total_count % 2 == 0 => {
				let index_one = even/2;
				let index_two = index_one - 1;
				return (n[index_one] + n[index_two]) as f64 / 2.0
			},
			odd => {
				return n[odd/2] as f64
			}
		}
		// println!("{}", 7/2);
		// return 0.0
    }
}
