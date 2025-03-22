use rate_my_sudoku::Sudoku;

fn main() {
    let default_filled_cells = 20;
    let args: Vec<String> = std::env::args().collect();
    let filled_cells = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(default_filled_cells)
    } else {
        default_filled_cells
    };
    let sudoku = Sudoku::generate(filled_cells);
    println!("{}", sudoku.serialized());
}
