use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "waydash",
        options,
        Box::new(|_cc| Ok(Box::new(App))), // <- Ok(...)
    )
}

#[derive(Default)]
struct App;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("waydash");
            ui.label("stub minimal — on branchera loggerd plus tard");
        });
    }
}
