
use eframe::egui;

use super::components::{ChargeTo, PcHeader, Paint, Parts};

#[derive(Debug, Default)]
pub struct ExtrasSheet {
    status: String,
    charge: ChargeTo,
    paint: Paint,
    parts: Parts,
    notes: String,
}

impl ExtrasSheet {
    fn init(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;

        // styles    
        ctx.set_style({
            let mut style: egui::Style = (*ctx.style()).clone();
    
            style.spacing.window_margin = egui::style::Margin::same(100.);

            style
        });

        Self::default()
    }

    pub fn new() -> eframe::AppCreator {
        Box::new(|cc| Box::new(Self::init(cc)))
    }
}

impl eframe::App for ExtrasSheet {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // header
        PcHeader::ui(ctx, frame);

        // footer
        egui::TopBottomPanel::bottom("footer")
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("© 2022 by High Steel Structures, LLC");
                });
            });

        // size panel
        egui::SidePanel::right("right_panel")
            .show(ctx, |ui| {
                self.charge.ui(ui);
                self.paint.ui(ui);
            });

        // main panel
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    self.parts.ui(ui);
                    ui.group(|ui| {
                        ui.label("<Notes>");
                        ui.text_edit_multiline(&mut self.notes)
                    });
                });
                ui.group(|ui| {
                    if ui.button("Save").clicked() {
                        self.status = "saved!".into();
                    }
                    ui.separator();
                    ui.label(&self.status);
                });
            });
    }
}

