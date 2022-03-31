use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let complement = target - nums[i];
            match map.get(&complement) {
                Some(&index) => return vec![i as i32, index],
                None => map.insert(nums[i], i as i32)
            };
        }

        unreachable!()
    }
}
