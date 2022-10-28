
#[derive(Debug, Default)]
pub struct Parts {
    parts: Vec<Part>
}

impl Parts {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("<Parts>");

            for part in &self.parts {
                ui.label(format!("{} ({})", part.name, part.qty));
            }
        });
    }
}

#[derive(Debug, Default)]
struct Part {
    name: String,
    qty: u32,

    // geometry
}
