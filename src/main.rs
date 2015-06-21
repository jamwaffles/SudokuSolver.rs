use std::collections::HashSet;

fn print_board(board: &Vec<Vec<u8>>) {
	for (i, row) in board.iter().enumerate() {
		for (j, &cell) in row.iter().enumerate() {
			print!("{}, ", cell);
		}

		println!("");
	}
}

fn gen_set() -> HashSet<u8> {
	let numbers: HashSet<u8> = [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();

	numbers
}

fn get_col_at(index: usize, grid: &Vec<Vec<u8>>) -> Vec<u8> {
	let mut values = Vec::new();

	for row in grid.iter() {
		values.push(row[index]);
	}

	values
}

// Get all non-zero numbers inside a block
fn get_block_for(x: usize, y: usize, grid: &Vec<Vec<u8>>) -> Vec<u8> {
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

	let mut filtered_block = Vec::new();

	// Filter out zeroes
	for &value in block.iter() {
		filtered_block.push(match value as u8 {
			0 => continue,
			_ => value
		});
	}

	filtered_block
}

fn block_possibilities_for_cell(block: &Vec<u8>) -> HashSet<u8> {
	let mut numbers = gen_set();

	for value in block.iter() {
		numbers.remove(value);
	}

	numbers
}

fn col_possibilities_for_cell(col: &Vec<u8>) -> HashSet<u8> {
	let mut numbers = gen_set();

	for value in col.iter() {
		numbers.remove(value);
	}

	numbers
}

fn row_possibilities_for_cell(row: &Vec<u8>) -> HashSet<u8> {
	let mut numbers = gen_set();

	for value in row.iter() {
		numbers.remove(value);
	}

	numbers
}

fn solve_board_iteration(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
	let mut solves = 0;

	let board = input.clone();
	let mut new_board = board.clone();

	for (i, row) in input.iter().enumerate() {
		for (j, &cell) in row.iter().enumerate() {
			let col = &get_col_at(j, &board);
			let block = &get_block_for(i, j, &board);

			if cell == 0 {
				let row_possibilities = row_possibilities_for_cell(row);
				let col_possibilities = col_possibilities_for_cell(col);
				let blk_possibilities = block_possibilities_for_cell(block);

				let row_col_possibilities: HashSet<u8> = row_possibilities.intersection(&col_possibilities).cloned().collect();
				let possibilities: HashSet<u8> = row_col_possibilities.intersection(&blk_possibilities).cloned().collect();

				if possibilities.len() == 1 {
					let cell_value = possibilities.iter().nth(0).unwrap();

					new_board[i][j] = *cell_value;

					solves += 1;
				}
			} else {
				continue;
			}
		}
	}

	println!("{} cells solved", solves);

	if solves == 0 {
		println!("No more cells solved");		
	}

	new_board
}

fn main() {
	let input_board = vec![
		vec![ 0, 4, 3, 5, 0, 0, 0, 0, 2 ],
		vec![ 0, 0, 2, 0, 0, 4, 0, 8, 3 ],
		vec![ 0, 1, 0, 0, 0, 0, 6, 0, 0 ],
		vec![ 0, 8, 0, 7, 3, 0, 0, 0, 5 ],
		vec![ 2, 6, 0, 0, 4, 5, 0, 0, 0 ],
		vec![ 1, 0, 0, 0, 0, 8, 0, 0, 0 ],
		vec![ 0, 7, 0, 3, 0, 0, 0, 6, 0 ],
		vec![ 0, 0, 0, 0, 0, 0, 0, 0, 0 ],
		vec![ 0, 0, 0, 6, 0, 7, 0, 0, 0 ]
	];

	let mut new_board: Vec<Vec<u8>> = input_board.clone();

	for i in 1..100 {
		new_board = solve_board_iteration(new_board);

		print_board(&new_board);
	}
}