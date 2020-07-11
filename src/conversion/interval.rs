use crate::conversion::note::note_to_string;
use crate::theory_primitive::interval::Interval;
use crate::theory_primitive::note::Note;
use std::collections::HashMap;

pub fn get_notes_for_intervals(root_note: &Note, intervals: &Vec<Interval>) -> Vec<String> {
    fn find_key_for_value<'a>(map: &'a HashMap<i32, Note>, value: &Note) -> Option<&'a i32> {
        map.iter()
            .find_map(|(key, &val)| if &val == value { Some(key) } else { None })
    }

    let notes_index: HashMap<i32, Note> = [
        (-15, Note::FFlatFlat),
        (-14, Note::CFlatFlat),
        (-13, Note::GFlatFlat),
        (-12, Note::DFlatFlat),
        (-11, Note::AFlatFlat),
        (-10, Note::EFlatFlat),
        (-9, Note::BFlatFlat),
        (-8, Note::FFlat),
        (-7, Note::CFlat),
        (-6, Note::GFlat),
        (-5, Note::DFlat),
        (-4, Note::AFlat),
        (-3, Note::EFlat),
        (-2, Note::BFlat),
        (-1, Note::F),
        (0, Note::C),
        (1, Note::G),
        (2, Note::D),
        (3, Note::A),
        (4, Note::E),
        (5, Note::B),
        (6, Note::FSharp),
        (7, Note::CSharp),
        (8, Note::GSharp),
        (9, Note::DSharp),
        (10, Note::ASharp),
        (11, Note::ESharp),
        (12, Note::BSharp),
        (13, Note::FSharpSharp),
        (14, Note::FSharpSharp),
        (15, Note::FSharpSharp),
        (16, Note::FSharpSharp),
        (17, Note::FSharpSharp),
        (18, Note::FSharpSharp),
        (19, Note::FSharpSharp),
    ]
    .iter()
    .cloned()
    .collect();

    let root_note_index = find_key_for_value(&notes_index, &root_note).unwrap();

    let notes_for_scale_degrees = intervals
        .into_iter()
        .map(|interval| {
            let enum_note = match interval {
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
            }
            .unwrap()
            .to_owned();

            note_to_string(&enum_note).to_owned()
        })
        .collect();

    notes_for_scale_degrees
}
