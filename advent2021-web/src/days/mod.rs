pub mod day0;

pub struct Example {
    pub title: String,
    pub input: String,
}

pub struct Day {
    pub title: String,
    pub description: String,
    pub examples: Vec<Example>,
}

pub struct Days {}

impl Days {
    pub fn get_days() -> Vec<Day> {
        vec![Day {
            title: "Example Day".into(),
            description: "Multiline String\nblah\nblah".into(),
            examples: vec![Example {
                title: "integers".into(),
                input: "1 + 2".into(),
            }],
        }]
    }
}
