use typst::doc::Frame;

mod shapes;
mod text;
mod update;
struct MyApp {
    page: Frame,
}

pub(crate) fn run(title: &str, page: Frame) {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(900.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        title,
        options,
        Box::new(|_cc| Box::new(MyApp { page })),
    ).unwrap()
}
