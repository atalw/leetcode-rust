impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
		// if close == n and open == n
		// can only add ( if open < n
		// can only add ) if close < open

		fn backtrack(s: String, n: i32, open: i32, close: i32) -> Vec<String> {
			let mut res: Vec<String> = Vec::new();

			if open == n && close == n { return vec![s] }

			if open < n {
				res.append(&mut backtrack(s.clone() + "(", n, open + 1, close));
			}

			if close < open {
				res.append(&mut backtrack(s.clone() + ")", n, open, close + 1));
			}

			res
		}

		backtrack("".to_string(), n, 0, 0)
    }
}
