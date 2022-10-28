

#[derive(Debug, PartialEq)]
pub enum ChargeTo { Job, CostCenter }

impl Default for ChargeTo {
    fn default() -> Self {
        Self::CostCenter
    }
}

impl ChargeTo {
    pub fn ui(&mut self, ui : &mut egui::Ui) {
        ui.group(|ui| {
            ui.radio_value(self, Self::Job, "Job");
            ui.radio_value(self, Self::CostCenter, "Cost Center");
        });
    }
}
