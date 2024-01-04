#![allow(unused_variables)]

use eframe::{
    egui::{self, FontData, FontDefinitions},
    epaint::FontFamily,
    run_native, App, CreationContext, NativeOptions, Result,
};

fn main() -> Result<()> {
    let native_options = NativeOptions::default();

    run_native(
        "My app",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

#[derive(Default)]
struct MyApp {}

impl MyApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        MyApp::set_fonts(cc);

        Self::default()
    }

    fn set_fonts(cc: &CreationContext<'_>) {
        let mut fonts = FontDefinitions::default();
        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert(
            "VictorMono-Regular".to_owned(),
            FontData::from_static(include_bytes!("../fonts/VictorMono-Regular.otf")),
        );

        // Put my font first (highest priority):
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "VictorMono-Regular".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("VictorMono-Regular".to_owned());

        cc.egui_ctx.set_fonts(fonts);
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hola mundo");
        });
    }
}
