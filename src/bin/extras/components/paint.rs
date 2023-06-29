
#[derive(Debug, Default)]
pub struct Paint {
    none: bool,
    blast: bool,
    single: bool,
}

impl Paint {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.checkbox(&mut self.none, "None");
            ui.checkbox(&mut self.blast, "Blast Only");
            ui.checkbox(&mut self.single, "Single Coat");
        });
    }
}
