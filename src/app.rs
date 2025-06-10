use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use egui::{vec2, Button, Color32, Painter, Rect, Response, Sense, Vec2};
use crate::game_state::GameState;

pub struct App {
    pub state: GameState,
    pub paused: bool,
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

            // Testing Area Start ---vvv

            

            // Testing Area End ---^^^

            // playing field
            let field_size = 1000f32; // TODO: make as big as window (resizing)
            let (response, painter) = ui.allocate_painter(
                Vec2::new(field_size, field_size),
                Sense::hover());

            // fill playing field: only renders subregion from view_x/view_y to
            // view_x + view_width / view_y + view_height
            let view_x = 0; // panel, TODO: adjusted by panning
            let view_y = 0; // panel, TODO: adjusted by panning
            let view_width = 50; // panel, TODO: adjusted by scrolling
            let view_height = view_width; // panel, TODO: adjusted by scrolling
            let view_size = field_size/view_width as f32;
            for y in view_y..(view_y + view_height) {
                for x in view_x..(view_x + view_width) {
                    self.paint_cell(x, y, &response, &painter, view_size); // extracted to down below, TODO: refactor
                }
            }
        });
        // Game Logic hook in, loop every some ms
        if Duration::from_millis(20) > start.elapsed() {
            let pause = Duration::from_millis(20) - start.elapsed();
            sleep(pause);
        }
        if !self.paused {
            self.state.update()
        }
        ctx.request_repaint();
    }
}

impl App {
    fn paint_cell(&self, x: usize, y: usize, response: &Response, painter: &Painter, view_size: f32, ) {
        let cell_rect = Rect::from_min_size(
        response.rect.min + vec2(x as f32 * (view_size), y as f32 * (view_size)),
        Vec2::splat(view_size),
        );
        painter.rect_filled(cell_rect, 0.0, if self.state.field[x][y] == 1 {Color32::BLACK} else {Color32::WHITE});
    }
}