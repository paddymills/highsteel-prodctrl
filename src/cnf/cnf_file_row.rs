
use crate::cnf::cnf_file_index as index;

#[derive(Debug, Default)]
pub struct CnfFileRow {
    pub part: PartInfo,
    pub matl: MatlInfo,
    pub plant: Plant,
    pub program: String
}

impl From<&String> for CnfFileRow {
    fn from(row: &String) -> Self {
        row.split("\t").collect::<Vec<&str>>().into()
    }
}

impl From<Vec<&str>> for CnfFileRow {
    fn from(row: Vec<&str>) -> Self {
        Self {
            part:    row.to_vec().into(),
            matl:    row.to_vec().into(),
            plant:   row[index::get("plant")].into(),
            program: row[index::get("program")].into()
        }
    }
}

impl ToString for CnfFileRow {
    fn to_string(&self) -> String {
        let end = index::max_index();
        let mut vals = vec![""; (end as usize)+1];

        let part_qty = format!("{:}", self.part.qty);
        let matl_qty = format!("{:.3}", self.matl.qty);

        vals[index::get("part-name")] = &self.part.mark;
        vals[index::get("part-job")]  = &self.part.job;
        vals[index::get("part-wbs")]  = &self.part.wbs;
        vals[index::get("part-qty")]  = &part_qty;

        vals[index::get("matl-name")] = &self.matl.material_master;
        vals[index::get("matl-wbs")]  = &self.matl.wbs;
        vals[index::get("matl-qty")]  = &matl_qty;
        vals[index::get("matl-loc")]  = &self.matl.loc;

        vals[index::get("plant")]     = &self.plant.as_str();
        vals[index::get("program")]   = &self.program;


        for (&k, &v) in index::ADDL.entries() {
            vals[k as usize] = v;
        }

        vals.join("\t").into()
    }
}

#[derive(Debug, Default)]
pub struct PartInfo {
    pub mark: String,
    pub job: String,
    pub wbs: String,
    pub qty: u32,
}

impl From<Vec<&str>> for PartInfo {
    fn from(row: Vec<&str>) -> Self {
        Self {
            mark: row[index::get("part-name")].into(),
            job:  row[index::get("part-job")].into(),
            wbs:  row[index::get("part-wbs")].into(),
            qty:  row[index::get("part-qty")].parse().expect("Failed ot parse part qty"),
        }
    }
}

#[derive(Debug, Default)]
pub struct MatlInfo {
    pub material_master: String,
    pub wbs: String,
    pub qty: f32,
    pub loc: String,
}

impl From<Vec<&str>> for MatlInfo {
    fn from(row: Vec<&str>) -> Self {
        Self {
            material_master: row[index::get("matl-name")].into(),
            wbs:             row[index::get("matl-wbs")].into(),
            qty:             row[index::get("matl-qty")].parse().expect("Failed ot parse part qty"),
            loc:             row[index::get("matl-loc")].into()
        }
    }
}

#[derive(Debug)]
pub enum Plant {
    Lancaster,
    Williamsport
}

impl Plant {
    pub fn as_str(&self) -> &'_ str {
        match self {
            Plant::Lancaster    => "HS01",
            Plant::Williamsport => "HS02",
        }
    }
}

impl From<&str> for Plant {
    fn from(plant: &str) -> Self {
        match plant {
            "HS01" => Plant::Lancaster,
            "HS02" => Plant::Williamsport,
            p      => panic!("Unexpected plant code: {}", p)
        }
    }
}

impl ToString for Plant {
    fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}

impl Default for Plant {
    fn default() -> Self {
        Plant::Lancaster
    }
}
