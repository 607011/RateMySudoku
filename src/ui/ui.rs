#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rate_my_sudoku::{EMPTY, Resolution, Strategy, StrategyResult, Sudoku, SudokuError, Unit};

use eframe::Storage;
use eframe::egui;
use egui::{Color32, Event, FontId, Pos2, Rect, Stroke, Vec2};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
static APP_NAME: &str = "Sudokui";

enum State {
    CalculateNotes,
    TryingStrategy,
    ApplyingStrategy,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppSettings {
    sudoku_string: String,
}

pub struct SudokuApp {
    settings: AppSettings,
    sudoku: Sudoku,
    strategy_result: StrategyResult,
    state: State,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for SudokuApp {
    fn default() -> Self {
        SudokuApp::new()
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for SudokuApp {
    fn default() -> Self {
        let mut sudoku = Sudoku::new();
        let sudoku_string =
            "008000063030000000000047120006000000001830400000901700000408031000500204200000000";
        sudoku.set_board_string(sudoku_string);
        Self {
            settings: AppSettings {
                sudoku_string: sudoku_string.to_string(),
            },
            sudoku,
            strategy_result: StrategyResult::empty(),
            state: State::CalculateNotes,
        }
    }
}

impl SudokuApp {
    #[cfg(not(target_arch = "wasm32"))]
    fn new() -> Self {
        SudokuApp {
            settings: AppSettings {
                sudoku_string: String::new(),
            },
            sudoku: Sudoku::new(),
            strategy_result: StrategyResult::empty(),
            state: State::CalculateNotes,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
        Default::default()
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        // Define colors and styles
        let background_color = Color32::from_rgb(250, 250, 250);
        let grid_color = Color32::from_rgb(180, 180, 180);
        let thick_line_color = Color32::BLACK;
        let filled_cell_color = Color32::from_rgb(235, 235, 235);
        let text_color = Color32::BLACK;
        let notes_color = Color32::from_gray(100);
        let shade_color = Color32::from_gray(170);

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
                match &self.strategy_result.removals.unit_index {
                    None => {
                        if sudoku.get_num(row, col) != EMPTY {
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
                    Some(unit) => match self.strategy_result.removals.unit {
                        None => {}
                        Some(Unit::Row) => {
                            if !unit.contains(&row) {
                                painter.rect_filled(
                                    Rect::from_min_size(
                                        Pos2::new(
                                            response.rect.min.x,
                                            response.rect.min.y + row as f32 * cell_size,
                                        ),
                                        Vec2::new(board_size, cell_size),
                                    ),
                                    0.0,
                                    shade_color,
                                );
                            }
                        }
                        Some(Unit::Column) => {
                            if !unit.contains(&col) {
                                painter.rect_filled(
                                    Rect::from_min_size(
                                        Pos2::new(
                                            response.rect.min.x + col as f32 * cell_size,
                                            response.rect.min.y,
                                        ),
                                        Vec2::new(cell_size, board_size),
                                    ),
                                    0.0,
                                    shade_color,
                                );
                            }
                        }
                        Some(Unit::Box) => {
                            let box_row = row / 3;
                            let box_col = col / 3;
                            let box_index = (box_row * 3) + box_col;
                            if !unit.contains(&box_index) {
                                painter.rect_filled(
                                    Rect::from_min_size(
                                        Pos2::new(
                                            response.rect.min.x + box_col as f32 * 3.0 * cell_size,
                                            response.rect.min.y + box_row as f32 * 3.0 * cell_size,
                                        ),
                                        Vec2::new(3.0 * cell_size, 3.0 * cell_size),
                                    ),
                                    0.0,
                                    shade_color,
                                );
                            }
                        }
                    },
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

                if sudoku.get_num(row, col) != EMPTY {
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
                        if sudoku.get_candidates(row, col).contains(&n) {
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

                            if highlight_affected && !highlight_about_to_be_removed {
                                let highlight_rect = Rect::from_center_size(
                                    note_pos,
                                    Vec2::new(note_size * 0.8, note_size * 0.8),
                                );
                                painter.rect_filled(
                                    highlight_rect,
                                    2.0,
                                    Color32::from_rgb(200, 255, 200), // Light green
                                );
                            } else if highlight_about_to_be_removed && !highlight_affected {
                                let highlight_rect = Rect::from_center_size(
                                    note_pos,
                                    Vec2::new(note_size * 0.8, note_size * 0.8),
                                );
                                painter.rect_filled(
                                    highlight_rect,
                                    2.0,
                                    Color32::from_rgb(255, 200, 200), // Light red
                                );
                            } else if highlight_about_to_be_removed {
                                let highlight_rect = Rect::from_center_size(
                                    note_pos,
                                    Vec2::new(note_size * 0.8, note_size * 0.8),
                                );
                                painter.rect_filled(
                                    highlight_rect,
                                    2.0,
                                    Color32::from_rgb(199, 240, 70), // Yellow green
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
        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load").clicked()
                        || ctx.input(|i| i.key_pressed(egui::Key::O) && i.modifiers.ctrl)
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            if path.extension().is_some_and(|ext| ext == "bin") {
                                match std::fs::read(&path) {
                                    Ok(content) => match Sudoku::from_binary(&content) {
                                        Ok(sudoku) => {
                                            self.sudoku = sudoku;
                                            self.settings.sudoku_string =
                                                self.sudoku.to_board_string();
                                            self.state = State::CalculateNotes;
                                            self.strategy_result.clear();
                                        }
                                        Err(err) => {
                                            log::error!("Failed to parse binary: {}", err)
                                        }
                                    },
                                    Err(err) => log::error!("Failed to load file: {}", err),
                                }
                            } else if path.extension().is_some_and(|ext| ext == "zst") {
                                match std::fs::read(&path) {
                                    Ok(content) => match Sudoku::from_zstd(&content) {
                                        Ok(sudoku) => {
                                            self.sudoku = sudoku;
                                            self.settings.sudoku_string =
                                                self.sudoku.to_board_string();
                                            self.state = State::CalculateNotes;
                                            self.strategy_result.clear();
                                        }
                                        Err(err) => {
                                            log::error!("Failed to parse binary: {}", err)
                                        }
                                    },
                                    Err(err) => log::error!("Failed to load file: {}", err),
                                }
                            } else {
                                match std::fs::read_to_string(&path) {
                                    Ok(content) => {
                                        if path.extension().is_some_and(|ext| ext == "json") {
                                            match Sudoku::from_json(&content) {
                                                Ok(sudoku) => {
                                                    self.sudoku = sudoku;
                                                    self.settings.sudoku_string =
                                                        self.sudoku.to_board_string();
                                                    self.state = State::CalculateNotes;
                                                    self.strategy_result.clear();
                                                }
                                                Err(err) => {
                                                    log::error!("Failed to parse JSON: {}", err)
                                                }
                                            }
                                        } else if let Ok(board_string) =
                                            self.sudoku.set_board_string(&content)
                                        {
                                            self.settings.sudoku_string = board_string.clone();
                                            self.state = State::CalculateNotes;
                                            self.strategy_result.clear();
                                        } else {
                                            log::error!("Invalid Sudoku board in file");
                                        }
                                    }
                                    Err(err) => log::error!("Failed to load file: {}", err),
                                }
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("Save as ...").clicked()
                        || ctx.input(|i| {
                            i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift
                        })
                    {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            let content: Vec<u8> =
                                if path.extension().is_some_and(|ext| ext == "json") {
                                    let mut sudoku = self.sudoku.clone();
                                    if !sudoku.has_candidates() {
                                        sudoku.calc_candidates();
                                    }
                                    sudoku.to_json().into()
                                } else if path.extension().is_some_and(|ext| ext == "bin") {
                                    let mut sudoku = self.sudoku.clone();
                                    if !sudoku.has_candidates() {
                                        sudoku.calc_candidates();
                                    }
                                    sudoku.to_binary()
                                } else if path.extension().is_some_and(|ext| ext == "zst") {
                                    let mut sudoku = self.sudoku.clone();
                                    if !sudoku.has_candidates() {
                                        sudoku.calc_candidates();
                                    }
                                    sudoku.to_zstd()
                                } else {
                                    self.sudoku.to_board_string().into()
                                };
                            if let Err(err) = std::fs::write(&path, content) {
                                log::error!("Failed to save file: {}", err);
                            } else {
                                log::info!("Saved file to {}", path.display());
                            }
                        }
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input(|i| {
                i.events.iter().any(|e| match e {
                    Event::Paste(board_string) => {
                        log::info!("Pasted sudoku: {}", &board_string);
                        if let Ok(board_string) = self.sudoku.set_board_string(board_string) {
                            self.settings.sudoku_string = board_string.clone();
                            self.state = State::CalculateNotes;
                            self.strategy_result.clear();
                            return true;
                        }
                        false
                    }
                    Event::Copy => {
                        log::info!("Use the 'Copy to clipboard' button");
                        true
                    }
                    Event::Key { key, pressed, .. } if *pressed => {
                        match *key {
                            egui::Key::ArrowRight if self.sudoku.unsolved() => {
                                self.proceed();
                            }
                            egui::Key::ArrowLeft => {
                                self.sudoku.prev_step();
                                self.state = State::CalculateNotes;
                                self.strategy_result.clear();
                                self.proceed();
                            }
                            _ => {}
                        }
                        true
                    }
                    _ => false,
                })
            }) {
                ctx.request_repaint();
            }
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("|<<").clicked() {
                        self.sudoku.restore();
                        self.state = State::CalculateNotes;
                        self.strategy_result.clear();
                        ctx.request_repaint();
                    }
                    if ui.button("<").clicked() {
                        self.sudoku.prev_step();
                        self.state = State::CalculateNotes;
                        self.strategy_result.clear();
                        self.proceed();
                        ctx.request_repaint();
                    }
                    if ui.button(">").clicked() {
                        self.proceed();
                        ctx.request_repaint();
                    }
                    if ui.button(">>|").clicked() {
                        self.strategy_result.clear();
                        self.sudoku.solve_by_backtracking();
                        ctx.request_repaint();
                    }
                    if ui.button("Copy to clipboard").clicked() {
                        if ctx.input(|i| i.modifiers.shift) {
                            self.handle_clipboard_copy(&format!("{}", &self.sudoku), ctx);
                        } else {
                            self.handle_clipboard_copy(&self.sudoku.to_board_string(), ctx);
                        }
                    }
                    // Status information display
                    let status_text = if self.strategy_result.strategy != Strategy::None {
                        if self.strategy_result.removals.unit.is_some() {
                            format!(
                                "Strategy: {} in {} {:?}",
                                self.strategy_result.strategy,
                                self.strategy_result.removals.unit.as_ref().unwrap(),
                                self.strategy_result.removals.unit_index.as_ref().unwrap()
                            )
                        } else {
                            format!("Strategy: {}", self.strategy_result.strategy)
                        }
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
        log::info!("Saving settings: {:?}", self.settings);
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }
}

impl SudokuApp {
    #[cfg(not(target_arch = "wasm32"))]
    fn load(&mut self, storage: &dyn Storage) -> Result<String, SudokuError> {
        self.sudoku.set_board_string(
            "008000063030000000000047120006000000001830400000901700000408031000500204200000000",
        )?;
        if let Some(path) = eframe::storage_dir(APP_NAME) {
            log::info!("Trying to load saved sudoku from {}", path.display());
        }
        if let Some(settings) = eframe::get_value::<AppSettings>(storage, eframe::APP_KEY) {
            log::info!("Loaded sudoku from storage: {}", settings.sudoku_string);
            self.sudoku.set_board_string(&settings.sudoku_string)?;
        }
        self.settings.sudoku_string = self.sudoku.to_board_string();
        Ok(self.settings.sudoku_string.clone())
    }

    fn proceed(&mut self) {
        match self.state {
            State::CalculateNotes => {
                self.sudoku.calc_candidates();
                self.state = State::TryingStrategy;
            }
            State::TryingStrategy => {
                self.strategy_result = self.sudoku.next_step();
                log::info!("{:?}", self.strategy_result);
                self.state = State::ApplyingStrategy;
            }
            State::ApplyingStrategy => {
                let resolution: Resolution = self.sudoku.apply(&self.strategy_result);
                log::info!("{:?}", resolution);
                self.strategy_result.clear();
                self.state = State::TryingStrategy;
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn handle_clipboard_copy(&self, text: &str, _ctx: &egui::Context) {
        if let Some(window) = web_sys::window() {
            let _ = window.navigator().clipboard().write_text(text);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn handle_clipboard_copy(&self, text: &str, ctx: &egui::Context) {
        ctx.output_mut(|o| o.commands = vec![egui::OutputCommand::CopyText(text.to_string())]);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .format_timestamp(None)
        .format_target(false)
        .init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            let mut app = SudokuApp::default();
            if let Some(storage) = cc.storage {
                app.load(storage)?;
            }
            Ok(Box::new(app))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let _ = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(SudokuApp::new(cc)))),
            )
            .await;
    });
}
