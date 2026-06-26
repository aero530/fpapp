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
            .with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Financial Planner",
        options,
        Box::new(|cc| Ok(Box::new(app::FpApp::new(cc)))),
    )
}
