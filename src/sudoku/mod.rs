use std::collections::HashSet;
use std::fs::File;
use std::io::{ Read };

pub struct Board {
	board: [ [ u8; 9 ]; 9 ],
	pub iterations_to_solve: u16
}

impl Board {
	pub fn new(input_board: &[ [ u8; 9 ]; 9 ]) -> Board {
		Board {
			board: input_board.clone(),
			iterations_to_solve: 0
		}
	}

	pub fn print_board(&self) {
		for (i, row) in self.board.iter().enumerate() {
			println!("{} {} {} ┃ {} {} {} ┃ {} {} {}", row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8]);

			if i % 3 == 2 && i != 8 {
				println!("━━━━━━╋━━━━━━━╋━━━━━━");
			}
		}
	}

	pub fn solve(&mut self) {
		loop {
			self.iterations_to_solve += 1;

			match self.solve_board_iteration() {
				Some(updated_board) => self.board = updated_board,
				None => break
			};
		};
	}

	pub fn import(path: &str) -> Board {
		let mut file = File::open(&path).unwrap();
		let mut content = String::new();
		let mut grid = [ [ 0u8; 9 ]; 9 ]; 

		file.read_to_string(&mut content).unwrap();

		let mut index = 0;

		for c in content.chars() {
			match c {
				// An empty but valid cell
				' ' | '.' => {
					index += 1;
				},

				// Any other char
				_ => {
					// Look for an integer
					match c.to_string().parse::<u8>() {
						Ok(n) => {
							let col = index / 9;
							let row = index - (col * 9);

							grid[col][row] = n;

							index += 1;
						},

						// If it's a junk char like a cell divider or grid char, do nothing
						_ => continue
					}
				}
			}
		};

		Board {
			board: grid,
			iterations_to_solve: 0
		}
	}

	fn get_block_at(&self, x: usize, y: usize) -> HashSet<u8> {
		// Figure out top left corner of block
		// Thanks to http://stackoverflow.com/a/13082705/383609 because I'm a dumbass
		let left = x - (x % 3);
		let top = y - (y % 3);

		let rows = vec![
			self.board[top].clone(),
			self.board[top + 1].clone(),
			self.board[top + 2].clone()
		];

		let mut block = HashSet::new();

		for row in rows.iter() {
			block.insert(row[left]);
			block.insert(row[left + 1]);
			block.insert(row[left + 2]);
		}

		block
	}

	fn get_possibilities_at(&self, x: usize, y: usize) -> HashSet<u8> {
		let mut possibilities: HashSet<u8> = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ].iter().cloned().collect();

		let block_values: HashSet<u8> = self.get_block_at(x, y);
		let column_values: HashSet<u8> = self.board.iter().map(|row| row[x]).collect();
		let row_values: HashSet<u8> = self.board[y].iter().cloned().collect();

		possibilities = possibilities.difference(&block_values).cloned().collect();
		possibilities = possibilities.difference(&column_values).cloned().collect();
		possibilities = possibilities.difference(&row_values).cloned().collect();

		possibilities
	}

	fn solve_board_iteration(&self) -> Option<[ [ u8; 9 ]; 9 ]> {
		let mut solves = 0;
		let mut new_board = self.board.clone();

		for (y, row) in self.board.iter().enumerate() {
			for (x, &cell) in row.iter().enumerate() {
				// Already got a value for this cell
				if cell > 0 {
					continue;
				}

				let mut possibilities = self.get_possibilities_at(x, y);

				// See if the remaining possibilities can go anywhere else. If they can't, we've found the value for this cell
				if possibilities.len() > 1 {
					for poss in possibilities.clone().iter() {
						let mut multi_row: bool = false;
						let mut multi_col: bool = false;
						let mut multi_blk: bool = false;

						// Can this value go anywhere else in the row?
						for (row_x, value) in self.board[y].iter().enumerate() {
							if row_x == x {
								continue;
							}

							multi_row = self.get_possibilities_at(row_x, y).contains(poss);

							if multi_row {
								break;
							}
						}

						let mut col = Vec::new();

						for row in self.board.iter() {
							col.push(&row[y]);
						}

						// Can this value go anywhere else in the column?
						for (col_y, value) in col.iter().enumerate() {
							if col_y == y {
								continue;
							}

							multi_col = self.get_possibilities_at(x, col_y).contains(poss);

							if multi_col {
								break;
							}
						}

						let left = x - (x % 3);
						let top = y - (y % 3);

						// Can this value go anywhere in the block it's in?
						'columns: for block_y in (top..top + 3) {
							for block_x in (left..left + 3) {
								if (block_x == x && block_y == y) || self.board[block_y][block_x] > 0 {
									continue;
								}

								multi_blk = self.get_possibilities_at(block_x, block_y).contains(poss);

								// Break out of both nested loops
								if multi_blk {
									break 'columns;
								}
							}
						}

						// If this number can go in two or more places, it's not longer a possibility
						if multi_row && multi_col && multi_blk {
							possibilities.remove(poss);
						}
					}
				}

				// If there's only one possibility left, it must be the value for this cell
				if possibilities.len() == 1 {
					new_board[y][x] = *possibilities.iter().nth(0).unwrap();

					solves += 1;
				}
			}
		};

		if solves > 0 {
			Some(new_board)
		} else {
			None
		}
	}
}