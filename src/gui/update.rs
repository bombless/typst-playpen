use super::{shapes::Shapes, text::Text};
use super::{MyApp};
use egui::{Color32, PointerButton, InputState, Pos2};
use egui::containers::Frame;
use egui::style::Margin;

const CELL_WIDTH: f32 = 30.;
const LEFT_MARGIN: f32 = 30.;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let options = Frame {
            fill: Color32::DARK_GRAY,
            inner_margin: Margin { left: LEFT_MARGIN, right: 0., top: 0., bottom: 0. },
            ..Frame::default()
        };

        egui::CentralPanel::default().frame(options).show(ctx, |ui| {

            for page in &self.doc.pages {
                ui.draw_text(
                    &format!("{:?}", page.text()),
                    CELL_WIDTH * 10.,
                    10.,
                    18.,
                    Color32::BLACK,
                );
            }
        });
        
    }
}
