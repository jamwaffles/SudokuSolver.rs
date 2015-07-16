mod sudoku;

fn main() {
	let mut puzzle = sudoku::Board::import("./boards/board1.txt");

	println!("Original board:");
	println!("");

	puzzle.print_board();

	puzzle.solve();

	println!("Finished {} iterations", puzzle.iterations_to_solve);

	if puzzle.num_unsolved > 0 {
		println!("Puzzle unsolved; {} cells left", puzzle.num_unsolved);
	} else {
		println!("Solved!");
	}

	println!("");

	puzzle.print_board();
}