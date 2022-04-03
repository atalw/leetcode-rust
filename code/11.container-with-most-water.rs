use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
		let mut start = 0;
		let mut end = height.len() - 1;

		let mut max_area = 0;

		while start < end {
			let first_h = height[start];
			let second_h = height[end];
			let width = end - start;
			let area = cmp::min(first_h, second_h) * width as i32;

			if area > max_area { max_area = area };

			if first_h < second_h { start += 1 } else { end -= 1 }
		}

		return max_area
    }
}
