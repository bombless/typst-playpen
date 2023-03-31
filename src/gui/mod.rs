use typst::doc::Frame;
use egui::{FontFamily, FontDefinitions, FontData};
use std::path::PathBuf;
use std::fs::read;

mod shapes;
mod text;
mod update;
struct MyApp {
    page: Frame,
    debug: bool,
}

pub(crate) fn run(title: &str, page: Frame, font: PathBuf) {

    let content = read(&font).unwrap();

    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert("my_font".to_owned(),
    FontData::from_owned(content)); // .ttf and .otf supported

    // egui_ctx.set_fonts(fonts);

    // Put my font first (highest priority):
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());


    let x = page.width().to_pt() as f32;
    let y = page.height().to_pt() as f32;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(x, y)),
        ..Default::default()
    };
    eframe::run_native(
        title,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            Box::new(MyApp { page, debug: true })
        }),
    ).unwrap()
}
