
pub struct PcHeader {}

impl PcHeader {
    pub fn ui(ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header")
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    // menu
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.menu_button(egui::RichText::new("=").text_style(egui::TextStyle::Button), |ui| {
                            if ui.button("Quit").clicked() {
                                frame.close();
                            }
                        });
                    });

                    // title
                    let title = ui.vertical_centered(|ui| {
                        ui.heading("Production Control Extras Input");
                    });
    
                    // window controls
                    let win_ctrls = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(10.);  // without this, close_btn goes right to the edge
                        let close_btn = ui.button(egui::RichText::new("🗙").text_style(egui::TextStyle::Button));

                        // TODO: impl minimize/maximize
                        // let max_btn = ui.button(egui::RichText::new("🗖").text_style(egui::TextStyle::Button));
                        // let min_btn = ui.button(egui::RichText::new("🗕").text_style(egui::TextStyle::Button));

                        if close_btn.clicked() {
                            frame.close();
                        }
                    });

                    if title.response.hovered() && !win_ctrls.response.hovered() {
                        frame.drag_window();
                    }
                });
            });
    }
}