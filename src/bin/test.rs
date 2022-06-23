use regex;

fn main() {
    let job_re = regex::RegexSet::new(&[
        r"^\d{1,7}$",
        r"^\d{7}[[:alpha:]]$"
    ]).unwrap();
    let passes = vec!["1", "11", "116", "1160", "11602", "116025", "1160253", "1160253A", "1160253a"];
    let fails = vec!["a", "1a", "11a", "116a", "1160a", "11602a", "116025a", "11602531", "1160253A1", "1160253aa"];

    println!("====[   Passes   ]=========================");
    for test in passes {
        println!("test: {} -> {:?}", test, job_re.is_match(test));
    }
    println!("====[   Fails    ]=========================");
    for test in fails {
        println!("test: {} -> {:?}", test, job_re.is_match(test));
    }
}
