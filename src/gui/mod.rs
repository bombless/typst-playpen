use typst::doc::Frame;

mod shapes;
mod text;
mod update;
struct MyApp {
    page: Frame,
}

pub(crate) fn run(title: &str, page: Frame) {
    let x = page.width().to_pt() as f32;
    let y = page.height().to_pt() as f32;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(x, y)),
        ..Default::default()
    };
    eframe::run_native(
        title,
        options,
        Box::new(|_cc| Box::new(MyApp { page })),
    ).unwrap()
}
