use crate::conversion::note::note_to_string;
use crate::theory_primitive::interval::Interval;
use crate::theory_primitive::note::Note;

pub fn get_notes_for_intervals(root_note: &Note, intervals: &Vec<Interval>) -> Vec<String> {
    let root_note_index = index_by_note(root_note);

    let notes_for_scale_degrees = intervals
        .into_iter()
        .map(|interval| {
            let enum_note = match interval {
                Interval::Perfect1 => note_by_index(root_note_index.to_owned()),
                Interval::Perfect4 => note_by_index(root_note_index - 1),
                Interval::Perfect5 => note_by_index(root_note_index + 1),
                Interval::Minor2 => note_by_index(root_note_index - 5),
                Interval::Major2 => note_by_index(root_note_index + 2),
                Interval::Minor3 => note_by_index(root_note_index - 3),
                Interval::Major3 => note_by_index(root_note_index + 4),
                Interval::Minor6 => note_by_index(root_note_index - 4),
                Interval::Major6 => note_by_index(root_note_index + 3),
                Interval::Minor7 => note_by_index(root_note_index - 2),
                Interval::Major7 => note_by_index(root_note_index + 5),
            };

            note_to_string(&enum_note).to_owned()
        })
        .collect();

    notes_for_scale_degrees
}

fn note_by_index(index: i32) -> Note {
    return match index {
        -15 => Note::FFlatFlat,
        -14 => Note::CFlatFlat,
        -13 => Note::GFlatFlat,
        -12 => Note::DFlatFlat,
        -11 => Note::AFlatFlat,
        -10 => Note::EFlatFlat,
        -9 => Note::BFlatFlat,
        -8 => Note::FFlat,
        -7 => Note::CFlat,
        -6 => Note::GFlat,
        -5 => Note::DFlat,
        -4 => Note::AFlat,
        -3 => Note::EFlat,
        -2 => Note::BFlat,
        -1 => Note::F,
        0 => Note::C,
        1 => Note::G,
        2 => Note::D,
        3 => Note::A,
        4 => Note::E,
        5 => Note::B,
        6 => Note::FSharp,
        7 => Note::CSharp,
        8 => Note::GSharp,
        9 => Note::DSharp,
        10 => Note::ASharp,
        11 => Note::ESharp,
        12 => Note::BSharp,
        13 => Note::FSharpSharp,
        14 => Note::CSharpSharp,
        15 => Note::GSharpSharp,
        16 => Note::DSharpSharp,
        17 => Note::ASharpSharp,
        18 => Note::ESharpSharp,
        19 => Note::BSharpSharp,
        _ => panic!("Invalid index"),
    };
}

fn index_by_note(note: &Note) -> i32 {
    return match note {
        Note::FFlatFlat => -15,
        Note::CFlatFlat => -14,
        Note::GFlatFlat => -13,
        Note::DFlatFlat => -12,
        Note::AFlatFlat => -11,
        Note::EFlatFlat => -10,
        Note::BFlatFlat => -9,
        Note::FFlat => -8,
        Note::CFlat => -7,
        Note::GFlat => -6,
        Note::DFlat => -5,
        Note::AFlat => -4,
        Note::EFlat => -3,
        Note::BFlat => -2,
        Note::F => -1,
        Note::C => 0,
        Note::G => 1,
        Note::D => 2,
        Note::A => 3,
        Note::E => 4,
        Note::B => 5,
        Note::FSharp => 6,
        Note::CSharp => 7,
        Note::GSharp => 8,
        Note::DSharp => 9,
        Note::ASharp => 10,
        Note::ESharp => 11,
        Note::BSharp => 12,
        Note::FSharpSharp => 13,
        Note::CSharpSharp => 14,
        Note::GSharpSharp => 15,
        Note::DSharpSharp => 16,
        Note::ASharpSharp => 17,
        Note::ESharpSharp => 18,
        Note::BSharpSharp => 19,
    };
}
