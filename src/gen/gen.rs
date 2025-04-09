use clap::Parser;
use rate_my_sudoku::generator::{FillAlgorithm, SudokuGenerator, ThinningAlgorithm};
use std::io::Write;
use std::sync::mpsc;
use std::thread;

#[derive(clap::Parser, Debug)]
#[command(name = "sudokugen", version = "0.1.0", about = "Generate Sudokus")]
struct Cli {
    #[arg(short, long, default_value_t = FillAlgorithm::DiagonalThinOut, help = "Algorithm to use for generating Sudoku puzzles")]
    algorithm: FillAlgorithm,
    #[arg(long, default_value_t = ThinningAlgorithm::Mirrored, help = "Algorithm to use for thinning out the Sudoku puzzles")]
    thinning: ThinningAlgorithm,
    #[arg(
        short = 'n',
        long,
        default_value_t = 24,
        value_name = "N",
        help = "Number of filled cells in the Sudoku puzzle"
    )]
    max_filled_cells: usize,
    #[arg(
        short = 'e',
        long,
        help = "Minimum difficulty level for the Sudoku puzzle"
    )]
    min_effort: Option<f64>,
    #[arg(short = 't', long, help = "Number of threads to use for generation")]
    num_threads: Option<usize>,
    #[arg(short, long, help = "Enable logging")]
    logging: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    if let Some(ref filter) = cli.logging {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(filter))
            .format_timestamp(None)
            .format_target(false)
            .init();
    };
    let min_effort = cli.min_effort;
    let max_filled_cells = cli.max_filled_cells;
    let fill_algorithm = cli.algorithm;
    let thinning = cli.thinning;
    let thread_count = match cli.num_threads {
        Some(num_threads) => num_threads,
        None => num_cpus::get(),
    };
    log::info!(
        "Starting Sudoku generation with {} threads using the fill algorithm {} for a maximum of {} filled cells and a minimum effort of {} ...",
        thread_count,
        fill_algorithm,
        max_filled_cells,
        min_effort.unwrap_or(f64::MIN)
    );

    let (tx, rx) = mpsc::channel();
    let stdout_mutex = std::sync::Mutex::new(());
    for _ in 0..thread_count {
        let tx = tx.clone();
        thread::spawn(move || {
            loop {
                let generator =
                    SudokuGenerator::new(fill_algorithm, Some(thinning), max_filled_cells);
                for sudoku in generator {
                    let sudoku_string = sudoku.to_board_string();
                    let mut computer_sudoku = sudoku.clone();
                    let mut sudoku = sudoku;
                    if sudoku.solve_human_like() {
                        if let Some(min_effort) = min_effort {
                            if sudoku.effort() < min_effort {
                                continue;
                            }
                        }
                        computer_sudoku.solve_by_backtracking();
                        if sudoku == computer_sudoku {
                            tx.send((sudoku.effort(), sudoku_string)).unwrap();
                        } else {
                            panic!(
                                "Solutions differ; human-like solver:\n{}\nbacktracking:\n{}\noriginal board: {}",
                                sudoku, computer_sudoku, sudoku_string
                            );
                        }
                    } else if min_effort.is_none() {
                        tx.send((f64::INFINITY, sudoku_string)).unwrap();
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
        if difficulty != f64::INFINITY {
            println!("{:6.2} {}", difficulty, sudoku_string);
        } else {
            println!("     ? {}", sudoku_string);
        }
        std::io::stdout().flush().unwrap();
    }

    Ok(())
}
