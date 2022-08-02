
use std::fmt::{self, Display, Formatter};

const DEFAULT_ZONE: u8 = 2;
const HPS_ZONE: u8 = 3;

/// Material grade
#[derive(Debug)]
pub struct Grade {
    spec: String,
    grade: String,
    test: Test,
    zone: u8
}

impl Grade {
    pub fn new(_spec: &str, _grade: &str, _test: &str, mut zone: u8) -> Self {
        let mut spec  = String::from(_spec);
        let mut grade = String::from(_grade);
        let mut test  = _test.into();

        if zone == 0 {
            zone = DEFAULT_ZONE
        }

        match _spec {
            "A240 Type 304" => {
                spec  = "A240".into();
                grade = "304".into();
                test  = Test::NotApplicable;
            },
            "A240 Type 316" => {
                spec  = "A240".into();
                grade = "316".into();
                test  = Test::NotApplicable;
            },
            "A606-TYPE4" => {
                spec  = "A606".into();
                grade = "TYPE4".into();
                test  = Test::NotApplicable;
            },
            _ => ()
        }

        if grade.contains("HPS") {
            zone = HPS_ZONE;
        }

        Self { spec, grade, test, zone }
    }

    pub fn force_cvn(&self) -> String {
        match self.test {
            Test::None => format!("{}-{}{:}{}", self.spec, self.grade, Test::Charpy, self.zone),
            _          => format!("{:}", self)
        }
    }
}

impl Default for Grade {
    fn default() -> Self {
        Self {
            spec: "A709".into(),
            grade: "[unknown]".into(),
            test: Test::None,
            zone: DEFAULT_ZONE
        }
    }
}

impl Display for Grade {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.test {
            Test::None          => write!(f, "{}-{}", self.spec, self.grade),
            Test::NotApplicable => write!(f, "{}-{}", self.spec, self.grade),
            _                   => write!(f, "{}-{}{:}{}", self.spec, self.grade, self.test, self.zone)
        }
    }
}

#[derive(Debug)]
enum Test {
    Fcm,
    Charpy,
    None,
    NotApplicable
}

impl From<&str> for Test {
    fn from(test: &str) -> Self {
        match test {
            "FCM" => Test::Fcm,
            "T"   => Test::Charpy,
            _     => Test::None
        }
    }
}

impl Default for Test {
    fn default() -> Self {
        Test::None
    }
}

impl Display for Test {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let val = match self {
            Test::Fcm           => "F",
            Test::Charpy        => "T",
            Test::None          => "",
            Test::NotApplicable => ""
        };

        write!(f, "{}", val)
    }
}
