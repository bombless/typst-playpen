use typst::doc::Document;

mod shapes;
mod text;
mod update;
struct MyApp {
    doc: Document,
}

pub(crate) fn run(doc: Document) {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(900.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp { doc })),
    ).unwrap()
}
