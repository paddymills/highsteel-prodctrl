
mod cnf_file_row;

pub use cnf_file_row::CnfFileRow;

pub mod cnf_file_index {

    use phf::phf_map;

    pub const PART_NAME: usize = 0;
    pub const PART_JOB:  usize = 1;
    pub const PART_WBS:  usize = 2;
    pub const PART_QTY:  usize = 4;
    
    pub const MATL_NAME: usize = 6;
    pub const MATL_WBS:  usize = 7;
    pub const MATL_QTY:  usize = 8;
    pub const MATL_LOC:  usize = 10;
    
    pub const PLANT:     usize = 11;
    pub const PROGRAM:   usize = 12;

    pub static INDEX: phf::Map<&'static str, u32> = phf_map! {
        "part-name" =>  0,
        "part-job"  =>  1,
        "part-wbs"  =>  2,
        "part-qty"  =>  4,
        
        "matl-name" =>  6,
        "matl-wbs"  =>  7,
        "matl-qty"  =>  8,
        "matl-loc"  => 10,

        "plant"     => 11,
        "program"   => 12,
    };

    /// panicing version of `INDEX.get()`
    /// for when keys are known
    pub fn get(key: &str) -> usize {
        match INDEX.get(key) {
            Some(&val) => val as usize,
            None       => unreachable!()
        }
    }

    pub fn max_index() -> u32 {
        match INDEX.into_iter().max() {
            Some((_key, &val)) => val,
            None               => 0
        }
    }

    pub static ADDL: phf::Map<u32, &'static str> = phf_map! {
        3u32 => "PROD",
        5u32 => "EA",
        9u32 => "IN2",
    };
}