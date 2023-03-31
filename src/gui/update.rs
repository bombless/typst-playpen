use super::{shapes::Shapes, text::Text as _};
use super::{MyApp};
use egui::{Color32, PointerButton, InputState, Pos2};
use egui::containers::Frame;
use egui::style::Margin;
use typst::doc::FrameItem::Text;

const CELL_HEIGHT: f32 = 24.;
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

            for (point, item) in self.page.items() {
                let text = if let Text(text) = item {
                    text
                } else {
                    continue;
                };
                ui.draw_text(
                    &text.glyphs.iter().map(|x| x.c).collect::<String>(),
                    point.x.to_pt() as f32,
                    point.y.to_pt() as f32,
                    text.size.to_pt() as f32,
                    Color32::BLACK,
                );
            }

            self.debug = false;
        });
        
    }
}
