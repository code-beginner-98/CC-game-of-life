use core::fmt;
use std::{
    fmt::format, sync::{Arc, Mutex}
};
use egui::{Button, Color32, Frame, Pos2, Rect, Sense, Slider, Vec2};
use crate::game_state::GameState;

pub struct App {
    pub shared_state: Arc<Mutex<GameState>>,
    pub shared_paused: Arc<Mutex<bool>>,
    pub pan_offset: Vec2,
    pub zoom: f32,
    pub shared_speed: Arc<Mutex<u8>>,
    pub window_height: Option<f32>
}

// Going with egui/eframe might be the worst mistake I will make in a long time due
// to it being immediate mode, but I don't care. I will just bruteforce my way through
// and optimize the unliving shine out of this and then go to persistant mode. I know
// egui has it. It's a learning process, people, deal with it.
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // reads out temporary values so the state doesn't lock forever.
        let (field_width, field_height) = {
            let state = self.shared_state.lock().unwrap();
            (state.field.len(), state.field[0].len())
        };

        // sets window position to be offset to application window bottom-left by 2.5% of window height.
        let screen_height = ctx.available_rect().height();
        let settings_window_height = self.window_height.unwrap_or(screen_height);
        let left_border_to_screen_left = screen_height * 0.025;
        let top_border_to_screen_bottom = screen_height * 0.975 - settings_window_height;
        let settings_window_position_fixed = Pos2::new(left_border_to_screen_left, top_border_to_screen_bottom);

        let settings_window = egui::Window::new("Settings")
            .fixed_pos(settings_window_position_fixed)
            .show(ctx, |ui| {
                if let Ok(mut speed) = self.shared_speed.lock() {
                    ui.add(Slider::new(&mut *speed, 1..=5).text("Simulation Speed"));
                }
                if let Ok(mut paused) = self.shared_paused.lock() {
                    let button_state = if *paused { "Resume"} else {"Pause"};
                    if ui.add(Button::new(button_state)).clicked() {
                        *paused = !*paused;
                    }
                }
                ui.label(format!("Zoom: {}", self.zoom));
        });
            
        // stores height for future use.
        self.window_height = settings_window.map(|response| response.response.rect.height());

        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::WHITE,
                ..Default::default()
            })
            .show(ctx, |ui| {
            // Here go all my UI elements

            // Playing field
            let panel_size_width = ctx.available_rect().width();
            let panel_size_height = ctx.available_rect().height();
            let (response, painter) = ui.allocate_painter(
                Vec2::new(panel_size_width, panel_size_height),
                Sense::click_and_drag());

            // Zoom
            let cell_size_standard = 10f32;
            self.zoom *= ctx.input(|i|i.zoom_delta());
            let min_zoom = (panel_size_height / field_height as f32 / cell_size_standard).max(panel_size_width / field_width as f32 / cell_size_standard);
            if self.zoom < min_zoom {
                self.zoom = min_zoom;
            }

            // Panning
            let cell_size = cell_size_standard * self.zoom;
            self.pan_offset += response.drag_delta();
            if self.pan_offset.x > 0.0 {self.pan_offset.x = 0.0};
            if self.pan_offset.y > 0.0 {self.pan_offset.y = 0.0};
            self.pan_offset.x = self.pan_offset.x.max((field_width as f32 *cell_size - panel_size_width) * - 1.0);
            self.pan_offset.y = self.pan_offset.y.max((field_height as f32 * cell_size - panel_size_height) * -1.0);

            let cell_vec = Vec2::new(cell_size, cell_size); // area that one cell occupies
            let top = 0.0f32.max(self.pan_offset.y * -1f32 / cell_size) as usize;
            let left = 0.0f32.max(self.pan_offset.x * -1f32 / cell_size) as usize;

            // Paint cells
            if let Ok(state) = self.shared_state.lock() {
                for (ind_y, y) in (top..top+((panel_size_height/cell_size) as usize)).into_iter().enumerate() {
                    for (ind_x, x) in (left..left+((panel_size_width/cell_size) as usize)).into_iter().enumerate() {
                        let cell_pos = Pos2::new(ind_x as f32 * cell_size, ind_y as f32 * cell_size);
                        if state.field[x][y] == 1 {
                            painter.rect_filled(Rect::from_min_size(cell_pos, cell_vec), 0.0, Color32::BLACK);
                        }
                    }
                }
            }
        });

        ctx.request_repaint();
    }
}