use std::collections::HashSet;

fn print_board(board: &Vec<Vec<u8>>) {
	for (i, row) in board.iter().enumerate() {
		println!("{} {} {} ┃ {} {} {} ┃ {} {} {}", 
			row[0],
			row[1],
			row[2],
			row[3],
			row[4],
			row[5],
			row[6],
			row[7],
			row[8]
		);

		if i % 3 == 2 && i != 8 {
			println!("━━━━━━╋━━━━━━━╋━━━━━━");
		}
	}
}

fn get_col_at(index: usize, grid: &Vec<Vec<u8>>) -> HashSet<u8> {
	let mut values = HashSet::new();

	for row in grid.iter() {
		match row[index] as u8 {
			0 => continue,
			_ => values.insert(row[index])
		};
	}

	values
}

fn get_row_at(index: usize, grid: &Vec<Vec<u8>>) -> HashSet<u8> {
	let mut values = HashSet::new();

	for &value in grid[index].iter() {
		match value as u8 {
			0 => continue,
			_ => values.insert(value)
		};
	}

	values
}

// Get all non-zero numbers inside a block
fn get_block_at(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> HashSet<u8> {
	let mut block = Vec::new();

	// Figure out top left corner of block
	// Thanks to http://stackoverflow.com/a/13082705/383609 because I'm a dumbass
	let left = x - (x % 3);
	let top = y - (y % 3);

	let mut rows = Vec::new();

	rows.push(grid[top].clone());
	rows.push(grid[top + 1].clone());
	rows.push(grid[top + 2].clone());

	for row in rows.iter() {
		block.push(row[left].clone());
		block.push(row[left + 1].clone());
		block.push(row[left + 2].clone());
	}

	let mut filtered_block = HashSet::new();

	// Filter out zeroes
	for &value in block.iter() {
		match value as u8 {
			0 => continue,
			_ => filtered_block.insert(value)
		};
	}

	filtered_block
}

// Get possibilities for a particular cell in the board
fn get_possibilities_at(x: usize, y: usize, board: &Vec<Vec<u8>>) -> HashSet<u8> {
	let mut possibilities: HashSet<u8> = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ].iter().cloned().collect();

	let block_values = get_block_at(x, y, &board);
	let column_values = get_col_at(x, &board);
	let row_values = get_row_at(y, &board);

	// println!("    ({}, {}): {:?}", x, y, row_values);

	possibilities = possibilities.difference(&block_values).cloned().collect();
	possibilities = possibilities.difference(&column_values).cloned().collect();
	possibilities = possibilities.difference(&row_values).cloned().collect();

	possibilities
}

fn board_possibilities_field(input: &Vec<Vec<u8>>) -> Option<Vec<Vec<u8>>> {
	let mut solves = 0;
	let mut new_board = input.clone();

	for (y, row) in input.iter().enumerate() {
		for (x, &cell) in row.iter().enumerate() {
			// Already got a value for this cell
			if cell > 0 {
				continue;
			}

			let possibilities = get_possibilities_at(x, y, &input);
			let mut filtered_possibilities = possibilities.clone();

			// See if the remaining possibilities can go anywhere else. If they can't, we've found the value for this cell
			if possibilities.len() > 1 {
				for poss in possibilities.iter() {
					let mut multi_row: bool = false;
					let mut multi_col: bool = false;
					let mut multi_blk: bool = false;

					// Can this value go anywhere else in the row?
					for (row_x, value) in input[y].iter().enumerate() {
						if row_x == x {
							continue;
						}

						multi_row = get_possibilities_at(row_x, y, &input).contains(poss);

						if multi_row {
							break;
						}
					}

					let mut col = Vec::new();

					for row in input.iter() {
						col.push(&row[y]);
					}

					// Can this value go anywhere else in the column?
					for (col_y, value) in col.iter().enumerate() {
						if col_y == y {
							continue;
						}

						multi_col = get_possibilities_at(x, col_y, &input).contains(poss);

						if multi_col {
							break;
						}
					}

					let left = x - (x % 3);
					let top = y - (y % 3);

					// Can this value go anywhere in the block it's in?
					for block_y in (top..top + 3) {
						for block_x in (left..left + 3) {
							if (block_x == x && block_y == y) || input[block_y][block_x] > 0 {
								continue;
							}

							multi_blk = get_possibilities_at(block_x, block_y, &input).contains(poss);

							if multi_blk {
								break;
							}
						}

						if multi_blk {
							break;
						}
					}

					if multi_row && multi_col && multi_blk {
						filtered_possibilities.remove(poss);
					}
				}

				if filtered_possibilities.len() == 1 {
					*(new_board.get_mut(y).unwrap().get_mut(x).unwrap()) = *filtered_possibilities.iter().nth(0).unwrap();

					solves += 1;
				}
			} else if possibilities.len() == 1 {
				*(new_board.get_mut(y).unwrap().get_mut(x).unwrap()) = *possibilities.iter().nth(0).unwrap();

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

fn main() {
	let input_board = vec![
		vec![ 7,0,0,0,2,0,0,0,4 ],
		vec![ 8,0,3,0,4,0,9,0,7 ],
		vec![ 0,0,2,0,0,0,3,0,0 ],
		vec![ 0,7,0,0,0,0,0,0,0 ],
		vec![ 0,6,0,9,0,5,0,1,0 ],
		vec![ 0,0,0,0,0,0,0,8,0 ],
		vec![ 0,0,9,0,0,0,6,0,0 ],
		vec![ 4,0,6,0,5,0,8,0,1 ],
		vec![ 5,0,0,0,3,0,0,0,2 ],
	];

	// let field = board_possibilities_field(&input_board);

	let mut new_board: Vec<Vec<u8>> = input_board.clone();

	println!("Original problem:\n");

	print_board(&new_board);

	println!("");

	let mut num_iterations = 1;

	loop {
		match board_possibilities_field(&new_board) {
			Some(updated) => {
				new_board = updated;
			},
			None => {
				println!("Solution complete:\n");

				print_board(&new_board);

				break;
			}
		};

		num_iterations += 1;
	};

	println!("");
	println!("Solved problem in {} iterations", num_iterations);
}