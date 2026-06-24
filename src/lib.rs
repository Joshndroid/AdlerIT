mod app;
pub mod hash;
mod theme;

use anyhow::Result;

/// Launch the native AdlerIt desktop window.
pub fn run_gui() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("AdlerIt")
            .with_inner_size([760.0, 520.0])
            .with_min_inner_size([560.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "AdlerIt",
        options,
        Box::new(|cc| Ok(Box::new(app::AdlerApp::new(cc)))),
    )
    .map_err(|error| anyhow::anyhow!(error.to_string()))
}
