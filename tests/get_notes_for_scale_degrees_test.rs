use music_theory_swiss_army_knife::interval::Interval;
use music_theory_swiss_army_knife::note::Note;
use music_theory_swiss_army_knife::scale;

static MAJOR_SCALE_INTERVALS: [Interval; 7] = [
    Interval::Perfect1,
    Interval::Major2,
    Interval::Major3,
    Interval::Perfect4,
    Interval::Perfect5,
    Interval::Major6,
    Interval::Major7,
];

#[test]
fn get_c_major_scale_should_return_c_major_scale() {
    let actual = scale::get_notes_for_scale_degrees(&Note::C, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["C", "D", "E", "F", "G", "A", "B"];

    assert_eq!(actual, expected);
}

#[test]
fn get_d_major_scale_should_return_c_major_scale() {
    let actual = scale::get_notes_for_scale_degrees(&Note::D, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["D", "E", "F♯", "G", "A", "B", "C♯"];

    assert_eq!(actual, expected);
}

#[test]
fn get_e_major_scale_should_return_c_major_scale() {
    let actual = scale::get_notes_for_scale_degrees(&Note::E, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["E", "F♯", "G♯", "A", "B", "C♯", "D♯"];

    assert_eq!(actual, expected);
}

#[test]
fn get_f_major_scale_should_return_c_major_scale() {
    let actual = scale::get_notes_for_scale_degrees(&Note::F, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["F", "G", "A", "B♭", "C", "D", "E"];

    assert_eq!(actual, expected);
}
