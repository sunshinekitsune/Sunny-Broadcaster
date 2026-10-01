mod osc;

use eframe::egui;
use std::error::Error;

#[derive(Default)]
struct SunnyBroadcasterApp;

impl SunnyBroadcasterApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for SunnyBroadcasterApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |_ui| {});
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let icon_bytes = include_bytes!("../assets/icon.png");
    let icon = eframe::icon_data::from_png_bytes(icon_bytes)?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Sunny Broadcaster")
            .with_icon(icon)
            .with_inner_size([400.0, 220.0])
            .with_min_inner_size([300.0, 160.0])
            .with_resizable(true),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "com.sunshinekitsune.sunny-broadcaster",
        options,
        Box::new(|cc| Ok(Box::new(SunnyBroadcasterApp::new(cc)))),
    )?;

    Ok(())
}
