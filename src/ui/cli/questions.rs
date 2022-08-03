
//! Common questions

use regex::{Regex, RegexSet};
use requestty::{OnEsc, prompt_one, Question};

/// Question to ask for a job number (with structure)
pub fn job() -> String {
    let validation_set = RegexSet::new( &[r"^\d{1,7}$", r"^\d{7}[[:alpha:]]$"] ).unwrap();
    let job_re = Regex::new( r"^\d{7}[[:alpha:]]$" ).unwrap();

    let question = Question::input("job")
        .on_esc(OnEsc::Terminate)
        .validate_on_key(move |input, _prev_ans| validation_set.is_match(input) )
        .validate(move |input, _prev_ans| {
            if job_re.is_match(input) {
                Ok(())
            } else {
                Err("Job must match pattern #######A".into())
            }
        })
        .build();

        prompt_one(question).unwrap().as_string().unwrap().into()
}

/// Question to ask for a shipment number
pub fn shipment() -> u32 {
    let question = Question::int("shipment")
        .on_esc(OnEsc::Terminate)
        .default(1)
        .validate(|ship, _prev_ans| {
            if ship > 0 {
                Ok(())
            } else {
                Err("Shipment must be a positive number".into())
            }
        })
        .build();

        prompt_one(question).unwrap().as_int().unwrap() as u32
}

#[cfg(test)]
mod regex_tests {
    
    #[test]
    fn validation_set() {
        let validation_set_re = regex::RegexSet::new(&[
            r"^\d{1,7}$",
            r"^\d{7}[[:alpha:]]$"
        ]).unwrap();
    
        // passes
        vec!["1", "11", "116", "1160", "11602", "116025", "1160253", "1160253A", "1160253a"]
            .iter()
            .for_each(|t| assert!(validation_set_re.is_match(t)));
        
        // failures
        vec!["a", "1a", "11a", "116a", "1160a", "11602a", "116025a", "11602531", "1160253A1", "1160253aa"]
            .iter()
            .for_each(|t| assert!(!validation_set_re.is_match(t)));
    }

    #[test]
    fn job() {
        let job_re = regex::Regex::new( r"^\d{7}[[:alpha:]]$" ).unwrap();

        // passes
        assert!(job_re.is_match("1200055c"));
        assert!(job_re.is_match("1180223A"));

        // failures
        assert!(!job_re.is_match("120055c"));
        assert!(!job_re.is_match("12000055c"));
        assert!(!job_re.is_match("1200055"));
    }
}
