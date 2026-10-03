use eframe::App;

#[derive(Default)]
pub struct AeroGraphApp;

impl App for AeroGraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AeroGraph Studio");
            ui.label("Massively Parallel Multi-Physics Simulation Canvas");
        });
    }
}
