mod sudoku;

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

	let mut puzzle = sudoku::Board::new(&input_board);

	puzzle.solve();

	println!("Puzzle solved, took {} iterations", puzzle.iterations_to_solve);
	println!("");

	puzzle.print_board();
}