#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod sudoku;
use sudoku::Sudoku;

use eframe::Storage;
use eframe::egui;
use egui::{Color32, Event, FontId, Pos2, Rect, Stroke, Vec2};
use serde::{Deserialize, Serialize};

static APP_KEY: &str = "sudokui";

#[derive(Default, Serialize, Deserialize)]
struct AppSettings {
    sudoku_string: String,
}

struct SudokuApp {
    settings: AppSettings,
    sudoku: Sudoku,
}

impl SudokuApp {
    fn new() -> Self {
        SudokuApp {
            settings: AppSettings {
                sudoku_string: String::new(),
            },
            sudoku: Sudoku::new(),
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
                // Handle cell selection - you can add code here to select a cell
                // and handle keyboard input for it
            }
        }

        let sudoku = &self.sudoku;

        // Draw filled cells for digits
        for row in 0..9 {
            for col in 0..9 {
                if sudoku.get_num(row, col) != 0 {
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
                        self.sudoku.next_step();
                    }
                    if ui.button(">>|").clicked() {
                        self.sudoku.solve_by_backtracking();
                    }
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
    fn load(&mut self, storage: &dyn Storage) {
        if let Some(path) = eframe::storage_dir(APP_KEY) {
            println!("Trying to load saved sudoku from {}", path.display());
        }
        if let Some(settings) = eframe::get_value::<AppSettings>(storage, eframe::APP_KEY) {
            self.sudoku.from_string(&settings.sudoku_string);
            self.sudoku.calc_all_notes();
        }
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
