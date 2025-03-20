#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod sudoku;
use sudoku::{EMPTY, Resolution, Strategy, StrategyResult, Sudoku};

use eframe::Storage;
use eframe::egui;
use egui::{Color32, Event, FontId, Pos2, Rect, Stroke, Vec2};
use serde::{Deserialize, Serialize};

static APP_KEY: &str = "sudokui";

enum State {
    None,
    TryingStrategy,
    ApplyingStrategy,
}

#[derive(Default, Serialize, Deserialize)]
struct AppSettings {
    sudoku_string: String,
}

struct SudokuApp {
    settings: AppSettings,
    sudoku: Sudoku,
    strategy_result: StrategyResult,
    state: State,
}

impl SudokuApp {
    fn new() -> Self {
        SudokuApp {
            settings: AppSettings {
                sudoku_string: String::new(),
            },
            sudoku: Sudoku::new(),
            strategy_result: StrategyResult::empty(),
            state: State::None,
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        // Define colors and styles
        let background_color = Color32::from_rgb(250, 250, 250);
        let grid_color = Color32::from_rgb(180, 180, 180);
        let thick_line_color = Color32::BLACK;
        let filled_cell_color = Color32::from_rgb(235, 235, 235);
        let text_color = Color32::BLACK;
        let notes_color = Color32::from_rgb(100, 100, 100);

        // Define stroke widths
        let thin_stroke = Stroke::new(1.0, grid_color);
        let thick_stroke = Stroke::new(2.0, thick_line_color);

        // Calculate size and spacing
        let board_size = ui.available_width().min(ui.available_height());
        let cell_size = board_size / 9.0;

        // Create a painter for custom drawing
        let (response, painter) = ui.allocate_painter(
            Vec2::new(board_size, board_size),
            egui::Sense::click_and_drag(),
        );

        // Draw the board background
        painter.rect_filled(
            Rect::from_min_size(response.rect.min, Vec2::new(board_size, board_size)),
            0.0,
            background_color,
        );

        // Handle mouse interaction to detect which cell was clicked
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            let rel_pos = mouse_pos - response.rect.min;
            let cell_x = (rel_pos.x / cell_size) as usize;
            let cell_y = (rel_pos.y / cell_size) as usize;
            if cell_x < 9 && cell_y < 9 {
                // Handle cell selection
            }
        }

        let sudoku = &self.sudoku;

        // Draw filled cells for digits
        for row in 0..9 {
            for col in 0..9 {
                if sudoku.get_num(row, col) != EMPTY {
                    // Draw filled background for cells with values
                    painter.rect_filled(
                        Rect::from_min_size(
                            Pos2::new(
                                response.rect.min.x + col as f32 * cell_size,
                                response.rect.min.y + row as f32 * cell_size,
                            ),
                            Vec2::new(cell_size, cell_size),
                        ),
                        0.0,
                        filled_cell_color,
                    );
                }
            }
        }

        // Draw the grid lines
        for i in 0..=9 {
            let line_stroke = if i % 3 == 0 {
                thick_stroke
            } else {
                thin_stroke
            };

            // Vertical lines
            painter.line_segment(
                [
                    Pos2::new(
                        response.rect.min.x + i as f32 * cell_size,
                        response.rect.min.y,
                    ),
                    Pos2::new(
                        response.rect.min.x + i as f32 * cell_size,
                        response.rect.min.y + board_size,
                    ),
                ],
                line_stroke,
            );

            // Horizontal lines
            painter.line_segment(
                [
                    Pos2::new(
                        response.rect.min.x,
                        response.rect.min.y + i as f32 * cell_size,
                    ),
                    Pos2::new(
                        response.rect.min.x + board_size,
                        response.rect.min.y + i as f32 * cell_size,
                    ),
                ],
                line_stroke,
            );
        }

        // Draw values and notes
        for row in 0..9 {
            for col in 0..9 {
                let cell_rect = Rect::from_min_size(
                    Pos2::new(
                        response.rect.min.x + col as f32 * cell_size,
                        response.rect.min.y + row as f32 * cell_size,
                    ),
                    Vec2::new(cell_size, cell_size),
                );

                if sudoku.get_num(row, col) != 0 {
                    // Draw the digit for filled cells
                    let digit = sudoku.get_num(row, col).to_string();
                    painter.text(
                        cell_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        digit,
                        FontId::proportional(cell_size * 0.7),
                        text_color,
                    );
                } else {
                    // Draw the notes in a 3x3 grid
                    let note_size = cell_size / 3.0;

                    for n in 1..=9 {
                        if sudoku.get_notes(row, col).contains(&n) {
                            let note_row = (n - 1) / 3;
                            let note_col = (n - 1) % 3;

                            let note_pos = Pos2::new(
                                cell_rect.min.x + note_col as f32 * note_size + note_size / 2.0,
                                cell_rect.min.y + note_row as f32 * note_size + note_size / 2.0,
                            );

                            // Check if this note is in the affected cells
                            let highlight_affected = self
                                .strategy_result
                                .removals
                                .candidates_affected
                                .iter()
                                .any(|cell| cell.row == row && cell.col == col && cell.num == n);

                            let highlight_about_to_be_removed = self
                                .strategy_result
                                .removals
                                .candidates_about_to_be_removed
                                .iter()
                                .any(|cell| cell.row == row && cell.col == col && cell.num == n);

                            if highlight_affected {
                                let highlight_rect = Rect::from_center_size(
                                    note_pos,
                                    Vec2::new(note_size * 0.8, note_size * 0.8),
                                );
                                painter.rect_filled(
                                    highlight_rect,
                                    2.0,
                                    Color32::from_rgb(200, 255, 200), // Light green
                                );
                            }
                            // Draw green background for affected notes
                            if highlight_about_to_be_removed && !highlight_affected {
                                let highlight_rect = Rect::from_center_size(
                                    note_pos,
                                    Vec2::new(note_size * 0.8, note_size * 0.8),
                                );
                                painter.rect_filled(
                                    highlight_rect,
                                    2.0,
                                    Color32::from_rgb(255, 200, 200), // Light red
                                );
                            }

                            painter.text(
                                note_pos,
                                egui::Align2::CENTER_CENTER,
                                n.to_string(),
                                FontId::proportional(note_size * 0.7),
                                notes_color,
                            );
                        }
                    }
                }
            }
        }
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input(|i| {
                i.events.iter().any(|e| {
                    if let Event::Paste(text) = e {
                        let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
                        if digits.len() != 81 {
                            return false;
                        }
                        self.sudoku.from_string(&digits);
                        self.sudoku.calc_all_notes();
                        self.settings.sudoku_string = digits;
                        true
                    } else {
                        false
                    }
                })
            }) {
                ctx.request_repaint();
            }
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("|<<").clicked() {
                        self.sudoku.restore();
                    }
                    if ui.button("<").clicked() {
                        self.sudoku.prev_step();
                    }
                    if ui.button(">").clicked() {
                        match self.state {
                            State::None | State::TryingStrategy => {
                                self.strategy_result = self.sudoku.next_step();
                                println!("{:?}", self.strategy_result);
                                self.state = State::ApplyingStrategy;
                            }
                            State::ApplyingStrategy => {
                                let resolution: Resolution =
                                    self.sudoku.apply(&self.strategy_result);
                                println!("{:?}", resolution);
                                self.strategy_result.clear();
                                self.state = State::TryingStrategy;
                            }
                        }
                        ctx.request_repaint();
                    }
                    if ui.button(">>|").clicked() {
                        self.strategy_result.clear();
                        self.sudoku.solve_by_backtracking();
                        ctx.request_repaint();
                    }

                    // Status information display
                    let status_text = if self.strategy_result.strategy != Strategy::None {
                        format!("Strategy: {}", self.strategy_result.strategy)
                    } else if self.sudoku.is_solved() {
                        std::fmt::format(format_args!(
                            "Solved! Effort: {:.1}",
                            self.sudoku.effort()
                        ))
                    } else {
                        "Ready".to_string()
                    };
                    ui.label(status_text);
                });
                self.draw(ui);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }
}

impl SudokuApp {
    fn load(&mut self, _storage: &dyn Storage) {
        if let Some(path) = eframe::storage_dir(APP_KEY) {
            println!("Trying to load saved sudoku from {}", path.display());
        }
        self.sudoku.from_string(
            "100700000000019030004800000020000050943000080608002900000000000092051700070040020",
        );
        // if let Some(settings) = eframe::get_value::<AppSettings>(storage, eframe::APP_KEY) {
        //     self.sudoku.from_string(&settings.sudoku_string);
        // }
        self.sudoku.calc_all_notes();
    }
}

impl Default for SudokuApp {
    fn default() -> Self {
        SudokuApp::new()
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sudokui",
        options,
        Box::new(|cc| {
            let mut app = SudokuApp::default();
            if let Some(storage) = cc.storage {
                app.load(storage);
            }
            Ok(Box::new(app))
        }),
    )
}
