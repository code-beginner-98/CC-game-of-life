use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use egui::{Button, Color32, Pos2, Rect, Sense, Vec2};
use crate::game_state::GameState;

pub struct App {
    pub state: GameState,
    pub paused: bool,
    pub pan_offset: Vec2,
    pub zoom: f32,
}

// Going with egui/eframe might be the worst mistake I will make in a long time due
// to it being immediate mode, but I don't care. I will just bruteforce my way through
// and optimize the unliving shine out of this and then go to persistant mode. I know
// egui has it. It's a learning process, people, deal with it.
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let start = Instant::now();
        egui::CentralPanel::default().show(ctx, |ui| {
            // Here go all my UI elements
            // TODO: put labels + buttons in seperate widget
            ui.label("Welome to the Game of Life");
            let button_state = if self.paused { "Resume"} else {"Pause"};
            if ui.add(Button::new(button_state)).clicked() {
                self.paused = !self.paused;
            }
            ui.label(format!("Zoom: {}", self.zoom));
            // Testing Area Start ---vvv

            

            // Testing Area End ---^^^

            // playing field
            let field_size = 1000f32; // TODO: make as big as window (resizing)
            let cell_size = 50f32*self.zoom; // panel, TODO: adjusted by scrolling
            let (response, painter) = ui.allocate_painter(
                Vec2::new(field_size, field_size),
                Sense::click_and_drag());

            // Zoom
            self.zoom *= ctx.input(|i|i.zoom_delta());
            if self.zoom < field_size / 50.0 / self.state.field.len() as f32 {
                self.zoom = field_size / 50.0 / self.state.field.len() as f32;
            }

            // Panning
            self.pan_offset += response.drag_delta();
            if self.pan_offset.x > 0.0 {self.pan_offset.x = 0.0};
            if self.pan_offset.y > 0.0 {self.pan_offset.y = 0.0};
            if self.pan_offset.x < ((self.state.field.len() as f32) * cell_size - field_size)*-1.0 {self.pan_offset.x = ((self.state.field.len() as f32) * cell_size - field_size)*-1.0}; // TODO: rewrite this crap.
            if self.pan_offset.y < ((self.state.field[0].len() as f32) * cell_size - field_size)*-1.0 {self.pan_offset.y = ((self.state.field.len() as f32) * cell_size - field_size)*-1.0}; // TODO: rewrite this crap.
            // ui.label(format!("Scene rect: {:#?}", &mut self.pan_offset));

            let cell_vec = Vec2::new(cell_size, cell_size);
            let top = if ((self.pan_offset.y * -1f32 / cell_size) as i32) < 0 {0} else {(self.pan_offset.y * -1f32 / cell_size) as usize}; // TODO: Rewrite as closure maybe?
            let left = if ((self.pan_offset.x * -1f32 / cell_size) as i32) < 0 {0} else {(self.pan_offset.x * -1f32 / cell_size) as usize}; // TODO: Rewrite as closure maybe?
            // TODO: add zooming - make range depend on zoomfactor
            for (ind_y, y) in (top..top+((field_size/cell_size) as usize)).into_iter().enumerate() {
                for (ind_x, x) in (left..left+((field_size/cell_size) as usize)).into_iter().enumerate() {
                    let cell_pos = Pos2::new(ind_x as f32 * cell_size, ind_y as f32 * cell_size);
                    if self.state.field[x][y] == 1 {
                        painter.rect_filled(Rect::from_min_size(cell_pos, cell_vec), 0.0, Color32::BLACK);
                    }
                }
            }
        });
        // Game Logic hook in, loop every some ms
        let refresh_rate = Duration::from_millis(20);
        if refresh_rate > start.elapsed() {
            let pause = refresh_rate - start.elapsed();
            sleep(pause);
        }
        if !self.paused {
            self.state.update()
        }
        ctx.request_repaint();
    }
}