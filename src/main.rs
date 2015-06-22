use std::collections::HashSet;

fn print_board(board: &Vec<Vec<u8>>) {
	for (i, row) in board.iter().enumerate() {
		for (j, &cell) in row.iter().enumerate() {
			print!("{}, ", cell);
		}

		println!("");
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

fn solve_board_iteration(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
	let mut solves = 0;

	// let grid = input.clone();
	let mut new_board = input.clone();

	for (y, row) in input.iter().enumerate() {
		for (x, &cell) in row.iter().enumerate() {
			// let col = &get_col_at(j, &input);
			// let block = &get_block_for(i, j, &input);

			if cell == 0 {
				let mut possibilities: HashSet<u8> = [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ].iter().cloned().collect();

				let block_values = get_block_at(x, y, &input);
				let column_values = get_col_at(x, &input);
				let row_values = get_row_at(y, &input);

				possibilities = possibilities.difference(&block_values).cloned().collect();
				possibilities = possibilities.difference(&column_values).cloned().collect();
				possibilities = possibilities.difference(&row_values).cloned().collect();

				if possibilities.len() == 1 {
					let cell_value = *possibilities.iter().nth(0).unwrap();

					*(new_board.get_mut(y).unwrap().get_mut(x).unwrap()) = cell_value;

					println!("Good solve at ({}, {}): {}", x, y, cell_value);

					solves += 1;
				}
			} else {
				continue;
			}
		}
	};

	println!("{} solves this iteration", solves);

	new_board
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

	let mut new_board: Vec<Vec<u8>> = input_board.clone();

	for i in 1..5 {
		new_board = solve_board_iteration(&new_board);

		print_board(&new_board);
	}
}