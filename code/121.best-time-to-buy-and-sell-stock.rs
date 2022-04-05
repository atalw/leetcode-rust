impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
		let mut profit = 0;
		let mut min_price = prices[0];

		for price in prices {
			profit = profit.max(price - min_price);
			min_price = min_price.min(price);
		}

		profit
    }
}
