use rate_my_sudoku::Sudoku;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .format_timestamp(None)
        .format_target(false)
        .init();

    let default_filled_cells: usize = 20;
    let args: Vec<String> = std::env::args().collect();
    let filled_cells = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(default_filled_cells)
    } else {
        default_filled_cells
    };
    let thread_count = num_cpus::get();
    let (tx, rx) = mpsc::channel();
    let stdout_mutex = std::sync::Mutex::new(());

    for _ in 0..thread_count {
        let tx = tx.clone();
        thread::spawn(move || {
            loop {
                if let Some(sudoku) = Sudoku::generate(filled_cells) {
                    let sudoku_string = sudoku.to_board_string();
                    let mut computer_sudoku = sudoku.clone();
                    let mut sudoku = sudoku;
                    if sudoku.solve_human_like() {
                        computer_sudoku.solve_by_backtracking();
                        if sudoku == computer_sudoku {
                            tx.send((sudoku.difficulty(), sudoku_string)).unwrap();
                        } else {
                            panic!(
                                "Solutions differ; human-like solver:\n{}\nbacktracking:\n{}\noriginal board: {}",
                                sudoku, computer_sudoku, sudoku_string
                            );
                        }
                    } else {
                        tx.send((-1.0, sudoku_string)).unwrap();
                    }
                }
            }
        });
    }

    // Drop the original sender to avoid keeping an extra reference
    drop(tx);

    // Print results from the channel
    while let Ok((difficulty, sudoku_string)) = rx.recv() {
        let _guard = stdout_mutex.lock().unwrap();
        if difficulty > 0.0 {
            println!("{:6.2} {}", difficulty, sudoku_string);
        } else {
            println!("     ? {}", sudoku_string);
        }
        std::io::stdout().flush().unwrap();
    }

    Ok(())
}
