use eframe::egui;
use crate::Simulation;

pub fn run_with_visualization(simulation: Simulation) {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new(cc))),
    );
}