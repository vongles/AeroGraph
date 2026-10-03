use eframe::NativeOptions;

mod app;
mod graph;
mod viewport;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("AeroGraph Studio")
            .with_inner_size([1280.0, 720.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "AeroGraph Studio",
        options,
        Box::new(|_cc| Box::new(app::AeroGraphApp::default())),
    )
}
