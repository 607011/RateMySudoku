use rate_my_sudoku::Sudoku;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .format_timestamp(None)
        .format_target(false)
        .init();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a serialized Sudoku board");
        return Err("No board provided".into());
    }
    if args[1].len() != 81 {
        println!("Please provide a string of length 81");
        return Err("Invalid board length".into());
    }
    let mut s0 = Sudoku::new();
    s0.set_board_string(&args[1])?;
    let start = std::time::Instant::now();
    s0.solve_puzzle();
    let duration = start.elapsed();
    println!(
        "Time to solve: {:.3} ms",
        1e-3 * duration.as_micros() as f64
    );
    let start = std::time::Instant::now();
    let mut s1 = Sudoku::new();
    s1.set_board_string(&args[1])?;
    s1.solve_by_backtracking();
    let duration = start.elapsed();
    println!(
        "For comparison: time to solve with backtracker: {:.3} ms",
        1e-3 * duration.as_micros() as f64
    );
    if s0 != s1 {
        println!("\nSOLUTIONS DIFFER\n");
        println!("Human-like solver:\n{}", s0);
        println!("Backtracking solver:\n{}", s1);
    }
    Ok(())
}
