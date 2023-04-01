use super::text::Text as _;
use super::MyApp;
use eframe::egui;
use egui::{Color32, FontFamily, Ui};
use typst::geom::Paint::Solid;
use egui::containers::Frame;
use egui::style::Margin;
use typst::doc::FrameItem::{Text, Group};
use typst::doc::{Frame as TypstFrame, TextItem};
use typst::geom::Point;

const LEFT_MARGIN: f32 = 30.;

fn render_text(ui: &mut Ui, text: &TextItem, point: &Point, display: bool) {
    for c in &text.glyphs {
        if display { print!("{}", c.c) }
    }

    let font_ptr = unsafe { std::mem::transmute::<_, usize>(text.font.data()) };
    let font_name = format!("font-{}", font_ptr);
    let family = FontFamily::Name(font_name.into());

    let Solid(color) = text.fill;
    let rgba_color = color.to_rgba();

    ui.draw_text(
        &text.glyphs.iter().map(|x| x.c).collect::<String>(),
        point.x.to_pt() as f32,
        point.y.to_pt() as f32,
        text.size.to_pt() as f32,
        family,
        Color32::from_rgba_unmultiplied(rgba_color.r, rgba_color.g, rgba_color.b, rgba_color.a),
    );
}

fn render_frame(ui: &mut Ui, frame: &TypstFrame, display: bool) {
    for (point, item) in frame.items() {
        match item {
            Text(text) => render_text(ui, text, point, display),
            Group(group) => render_frame(ui, &group.frame, display),
            _ => (),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {

        let options = Frame {
            fill: Color32::WHITE,
            inner_margin: Margin { left: LEFT_MARGIN, right: 0., top: 0., bottom: 0. },
            ..Frame::default()
        };

        egui::CentralPanel::default().frame(options).show(ctx, |ui| {
            render_frame(ui, &self.page, self.display);
            println!();
            self.display = false;
        });
        
    }
}
