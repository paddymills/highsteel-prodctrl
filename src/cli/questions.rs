
use regex::{Regex, RegexSet};
use requestty::{OnEsc, prompt_one, Question};

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