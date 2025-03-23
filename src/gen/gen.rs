use rate_my_sudoku::Sudoku;

fn main() {
    let default_filled_cells: usize = 20;
    let args: Vec<String> = std::env::args().collect();
    let filled_cells = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(default_filled_cells)
    } else {
        default_filled_cells
    };
    loop {
        if let Some(sudoku) = Sudoku::generate(filled_cells) {
            let sudoku_string = sudoku.serialized();
            let mut sudoku = sudoku;
            if sudoku.solve_human_like() {
                println!("{:6.2} {}", sudoku.difficulty(), sudoku_string);
            } else {
                println!("FAILED {}", sudoku_string);
            }
        }
    }
}
