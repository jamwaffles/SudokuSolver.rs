mod sudoku;

fn main() {
	let mut puzzle = sudoku::Board::import("./boards/board1.txt");

	println!("Original board:");
	println!("");

	puzzle.print_board();

	puzzle.solve();

	println!("Puzzle solved, took {} iterations", puzzle.iterations_to_solve);
	println!("");

	puzzle.print_board();
}