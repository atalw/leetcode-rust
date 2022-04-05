impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
		let mut min_price = prices[0];

		prices.iter().fold(0, |profit, &price| {
			min_price = min_price.min(price);
			profit.max(price - min_price)
		})

		// for price in prices {
		//     profit = profit.max(price - min_price);
		//     min_price = min_price.min(price);
		// }

		// profit
    }
}
