mod analyze;
mod app;
mod dashboard;
mod forms;
mod nav;
mod settings_view;
mod widgets;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Financial Planner")
            .with_inner_size([1280.0, 800.0])
            .with_icon(std::sync::Arc::new(load_icon())),
        ..Default::default()
    };
    eframe::run_native(
        "Financial Planner",
        options,
        Box::new(|cc| Ok(Box::new(app::FpApp::new(cc)))),
    )
}

fn load_icon() -> eframe::egui::IconData {
    let bytes = include_bytes!("../../resources/icons/256x256.png");
    let image = image::load_from_memory(bytes)
        .expect("failed to decode app icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    eframe::egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}
