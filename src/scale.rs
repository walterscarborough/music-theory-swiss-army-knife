use std::collections::HashMap;

use crate::interval::Interval;

#[derive(PartialEq, Debug, Clone)]
pub struct Scale {
    pub name: String,
    pub aliases: Vec<String>,
    pub intervals: Vec<Interval>,
}

pub fn get_notes_for_scale_degrees(root_note: &str, intervals: &Vec<Interval>) -> Vec<String> {

    fn find_key_for_value<'a>(map: &'a HashMap<i32, &'static str>, value: &str) -> Option<&'a i32> {
        map
            .iter()
            .find_map(|(key, &val)| if val == value { Some(key) } else { None })
    }

    let notes_index = [
        (-15, "F𝄫"),
        (-14, "C𝄫"),
        (-13, "G𝄫"),
        (-12, "D𝄫"),
        (-11, "A𝄫"),
        (-10, "E𝄫"),
        (-9, "B𝄫"),
        (-8, "F♭"),
        (-7, "C♭"),
        (-6, "G♭"),
        (-5, "D♭"),
        (-4, "A♭"),
        (-3, "E♭"),
        (-2, "B♭"),
        (-1, "F"),
        (0, "C"),
        (1, "G"),
        (2, "D"),
        (3, "A"),
        (4, "E"),
        (5, "B"),
        (6, "F♯"),
        (7, "C♯"),
        (8, "G♯"),
        (9, "D♯"),
        (10, "A♯"),
        (11, "E♯"),
        (12, "B♯"),
        (13, "F𝄪"),
        (14, "C𝄪"),
        (15, "G𝄪"),
        (16, "D𝄪"),
        (17, "A𝄪"),
        (18, "E𝄪"),
        (19, "B𝄪"),
    ]
        .iter()
        .cloned()
        .collect();

    let root_note_index = find_key_for_value(&notes_index, &root_note).unwrap();

    let notes_for_scale_degrees = intervals
        .into_iter()
        .map(|interval| {
            match interval {
                Interval::Perfect1 => notes_index.get(&root_note_index),
                Interval::Perfect4 => notes_index.get(&(root_note_index - 1)),
                Interval::Perfect5 => notes_index.get(&(root_note_index + 1)),
                Interval::Minor2 => notes_index.get(&(root_note_index - 5)),
                Interval::Major2 => notes_index.get(&(root_note_index + 2)),
                Interval::Minor3 => notes_index.get(&(root_note_index - 3)),
                Interval::Major3 => notes_index.get(&(root_note_index + 4)),
                Interval::Minor6 => notes_index.get(&(root_note_index - 4)),
                Interval::Major6 => notes_index.get(&(root_note_index + 3)),
                Interval::Minor7 => notes_index.get(&(root_note_index - 2)),
                Interval::Major7 => notes_index.get(&(root_note_index + 5)),
            }.unwrap().to_owned().to_owned()
        })
        .collect();

    notes_for_scale_degrees
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
                    Interval::Major2,
                    Interval::Major3,
                    Interval::Perfect4,
                    Interval::Perfect5,
                    Interval::Major6,
                    Interval::Major7,
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
