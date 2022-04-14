use std::ops::Sub;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Position {
	x: usize,
	y: usize 
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
		// let board: Vec<Vec<usize>> = Vec::with_capacity(n as usize);
		let mut board: Vec<Vec<usize>> = vec![vec![0; n as usize]; n as usize];
		// board[0][0] = 1;
		let mut solutions: Vec<Vec<Vec<usize>>> = Vec::new();
		for i in 0..n {
			board[0][i as usize] = 1;
			// println!("starting board: {:?}", board);
			let open_positions = Solution::get_open_positions(&mut board.clone());
			// println!("open positions: {:?}", open_positions);
			let mut res: Vec<Vec<Vec<usize>>> = Vec::new();
			match Solution::solve_n_queens_internal(&mut board.clone(), open_positions, Position{ x: 0, y: 0}, res) {
				Some(solved_boards) => {
					// println!("solved board: {:?}", solved_boards);
					for solution in solved_boards {
						solutions.push(solution.to_vec());
					}
				},
				None => {}
			};
			board[0][i as usize] = 0;
		}

		Solution::convert_to_output(solutions)
    }

	fn convert_to_output(solutions: Vec<Vec<Vec<usize>>>) -> Vec<Vec<String>> {
		let mut res: Vec<Vec<String>> = Vec::new();
		if solutions.len() == 0 { return res }

		let mut index = 0;
		for (i, solution) in solutions.iter().enumerate() {
			let mut solution_string: Vec<String> = Vec::with_capacity(solution.len());
			for row in solution {
				let mut s = "".to_string();
				for val in row {
					if *val == 0 { s.push('.') } else { s.push('Q') }
				}
				solution_string.push(s);
			}
			res.push(solution_string);
		}
		res
	}

	// Add padding when printing output
	fn indent(n: usize) {
		for i in 0..n {
			print!("    ");
		}
	}

	fn solve_n_queens_internal(mut board: &mut [Vec<usize>], open_positions: Vec<Position>, chosen_position: Position, mut res: Vec<Vec<Vec<usize>>>) -> Option<Vec<Vec<Vec<usize>>>> {
		// Solution::indent(chosen_position.y);
		// println!("board: {:?}", board);
		// Solution::indent(chosen_position.y);
		// println!("open positions: {:?}", open_positions);

		// last row
		if chosen_position.y == board.len() - 1 { 
			res.push(board.to_vec());
			return Some(res)
		} 
		if open_positions.is_empty() { return Some(res) }

		for i in 0..board.len() {
			// choose
			let p = Position { x: i, y: chosen_position.y + 1 };
			if !open_positions.contains(&p) { continue }

			board[p.y][p.x] = 1;

			// explore
			let temp_open_positions = Solution::get_open_positions(board);
			match Solution::solve_n_queens_internal(board, temp_open_positions, p, res.clone()) {
				Some(r) => { 
					res = r;
				}
				None => return None
			}

			// unchoose
			board[p.y][p.x] = 0;
		}

		Some(res)
	}

	fn get_open_positions(board: &mut [Vec<usize>]) -> Vec<Position> {
		let mut positions: Vec<Position> = Vec::new();

		for y in 0..board.len() {
			for x in 0..board[0].len() {
				positions.push(Position{ x, y });
			}
		}
		for y in 0..board.len() {
			for x in 0..board[0].len() {
				// println!("{:?} {:?}", x, y);
				let val = board[y][x];
				if val == 1 {
					let remove_positions = Solution::get_queen_attacks(board, Position { x, y });
					// println!("remove: {:?}", remove_positions);
					positions.retain(|x| !remove_positions.contains(x));
					// positions.drain_filter(|x| remove_positions.contains(x));
				}
			}
		}

		// println!("board: {:?}", board);
		// println!("positions: {:?}", positions);

		positions
	}

	// these are the positions queen can attack (only solving for unseen part of board)
	// left diagonal => x-1, y+1
	// right diagonal => x+1, y+1
	// column => x, y+1
	// row => x+1, y (y stays constant)
	fn get_queen_attacks(board: &[Vec<usize>], position: Position) -> Vec<Position> {
		let mut positions: Vec<Position> = Vec::new();
		positions.push(position);

		let mut x = position.x;
		let mut y = position.y;

		// get left diagonal
		while x > 0 && y < board.len() {
			x = x - 1;
			y = y + 1;
			let pos = Position { x, y };
			if x >= 0 && y < board.len() {
				positions.push(pos);
			}
		}

		x = position.x;
		y = position.y;

		// get right diagonal
		while x < board[0].len() && y < board.len() {
			x = x + 1;
			y = y + 1;
			let pos = Position { x, y };
			if x < board[0].len() && y < board.len() {
				positions.push(pos);
			}
		}

		x = position.x;
		y = 0; // queen can attack whole column

		// column
		while y < board.len() {
			let pos = Position { x, y };
			if y < board.len() {
				positions.push(pos);
			}
			x = x;
			y = y + 1;
		}

		x = 0; // queen can attack whole row
		y = position.y;

		// row 
		while x < board[0].len() {
			let pos = Position { x, y };
			if x < board[0].len() {
				positions.push(pos);
			}
			x = x + 1;
			y = y;
		}

		positions
	}
}
