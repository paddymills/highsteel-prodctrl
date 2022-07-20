use pbr::{MultiBar, ProgressBar, Pipe};

pub struct ProgressBars {
    pub jobs: ProgressBar<Pipe>,
    pub bom: ProgressBar<Pipe>,
    pub sndb: ProgressBar<Pipe>,
    pub dxf_sn: ProgressBar<Pipe>,
    pub dxf_fs: ProgressBar<Pipe>,
    
}

impl ProgressBars {
    pub fn new() -> Self {
        let mb = MultiBar::new();
        let mut jobs = mb.create_bar(0);
        let mut bom = mb.create_bar(0);
        let mut sndb = mb.create_bar(0);
        let mut dxf_sn = mb.create_bar(0);
        let mut dxf_fs = mb.create_bar(0);

        jobs.message("Jobs ");
        bom.message("Parts > Bom ");
        sndb.message("Parts > Sn ");
        dxf_sn.message("Parts > Dxf (Sn) ");
        dxf_fs.message("Parts > Dxf (fs) ");

        std::thread::spawn(move || { mb.listen(); });

        Self { jobs, bom, sndb, dxf_sn, dxf_fs }
    }

    pub fn inc_job(&mut self) {
        self.jobs.total += 1;

        self.tick_all();
    }

    #[allow(dead_code)]
    pub fn inc_part(&mut self) {
        self.bom.total += 1;
        self.sndb.total += 1;
        self.dxf_sn.total += 1;
        // self.dxf_fs.total += 1;

        self.tick_all();
    }

    pub fn tick_all(&mut self) {
        self.jobs.tick();
        self.bom.tick();
        self.sndb.tick();
        self.dxf_sn.tick();
        self.dxf_fs.tick();
    }
}
