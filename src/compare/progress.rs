
use nutmeg::Model;

#[derive(Debug, Default)]
pub struct GetJobs {
    index: usize,
    total: usize,
    message: String
}

impl GetJobs {
    pub fn grow(&self, i: usize) {
        self.total += i;
    }

    pub fn inc(&self, i: usize) {
        self.index += i;
    }
}

impl Model for GetJobs {
    fn render(&mut self, _width: usize) -> String {
        format!("{} {}/{}", self.message, self.index, self.total)
    }

    fn final_message(&mut self) -> String {
        format!("{} ... done", self.message)
    }
}

