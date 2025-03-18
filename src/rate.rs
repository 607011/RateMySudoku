mod sudoku;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a serialized Sudoku board");
        return;
    }
    if args[1].len() != 81 {
        println!("Please provide a string of length 81");
        return;
    }
    let mut s = sudoku::Sudoku::new();
    s.from_string(&args[1]);
    let start = std::time::Instant::now();
    s.solve_puzzle();
    let duration = start.elapsed();

    println!("Time to solve: {} µs", duration.as_micros());
}
