use crate::interval::Interval;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub struct Scale {
    pub name: String,
    pub aliases: Vec<String>,
    pub intervals: Vec<Interval>,
}

pub fn get(name: &str) -> Scale {
    let scale_dict = generate_scale_dict();

    let maybe_scale = scale_dict.get(name);

    let scale = match maybe_scale {
        Some(scale) => scale,
        None => panic!("Unable to find the scale: {:?}", name),
    };

    scale.clone()
}

pub fn get_all() -> Vec<Scale> {
    let scale_dict = generate_scale_dict();

    scale_dict.values().cloned().collect()
}

fn generate_scale_dict() -> HashMap<String, Scale> {
    [
        (
            String::from("major"),
            Scale {
                name: String::from("major"),
                aliases: vec![String::from("ionian")],
                intervals: vec![
                    Interval::Perfect1,
                    Interval::Minor2,
                    Interval::Major3,
                    Interval::Perfect4,
                    Interval::Perfect5,
                    Interval::Minor6,
                    Interval::Minor7,
                ],
            },
        ),
        (
            String::from("minor"),
            Scale {
                name: String::from("minor"),
                aliases: vec![String::from("aeolian")],
                intervals: vec![
                    Interval::Perfect1,
                    Interval::Major2,
                    Interval::Minor3,
                    Interval::Perfect4,
                    Interval::Perfect5,
                    Interval::Minor6,
                    Interval::Minor7,
                ],
            },
        ),
    ]
    .iter()
    .cloned()
    .collect()
}
