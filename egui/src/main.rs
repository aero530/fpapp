mod analyze;
mod app;
mod dashboard;
mod forms;
mod logger;
mod nav;
mod platform;
mod settings_view;
mod widgets;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let warnings = logger::init();
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
        Box::new(move |cc| Ok(Box::new(app::FpApp::new(cc, warnings)))),
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn load_icon() -> eframe::egui::IconData {
    let bytes = include_bytes!("../assets/icon-256.png");
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

/// Elements the hosting page is expected to provide — see `egui/web/index.html`.
#[cfg(target_arch = "wasm32")]
const CANVAS_ID: &str = "fpapp_canvas";
#[cfg(target_arch = "wasm32")]
const LOADING_ID: &str = "fpapp_loading";

/// Web entry point.  wasm-bindgen turns this into the module's start function,
/// so the page only has to `await init()`.
#[cfg(target_arch = "wasm32")]
fn main() {
    let warnings = logger::init();
    // Starting the renderer is async (WebGL context creation), so it cannot
    // block the start function.
    wasm_bindgen_futures::spawn_local(async move {
        match start_web(warnings).await {
            Ok(()) => set_loading_message(None),
            // The canvas stays blank on failure, so say why on the page itself
            // rather than only in the console.
            Err(message) => {
                log::error!("{}", message);
                set_loading_message(Some(&format!("Could not start the app: {}", message)));
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
async fn start_web(warnings: logger::WarningBuffer) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    let canvas = document()
        .and_then(|document| document.get_element_by_id(CANVAS_ID))
        .and_then(|element| element.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        .ok_or_else(|| format!("the page has no <canvas id=\"{}\">", CANVAS_ID))?;

    eframe::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(move |cc| Ok(Box::new(app::FpApp::new(cc, warnings)))),
        )
        .await
        .map_err(|err| {
            err.as_string()
                .unwrap_or_else(|| String::from("WebGL2 is unavailable or was refused"))
        })
}

#[cfg(target_arch = "wasm32")]
fn document() -> Option<web_sys::Document> {
    web_sys::window()?.document()
}

/// Replace the page's loading text, or remove it once the app is up.
#[cfg(target_arch = "wasm32")]
fn set_loading_message(message: Option<&str>) {
    let Some(element) = document().and_then(|document| document.get_element_by_id(LOADING_ID))
    else {
        return;
    };
    match message {
        Some(message) => element.set_text_content(Some(message)),
        None => element.remove(),
    }
}
