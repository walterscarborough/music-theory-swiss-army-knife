use music_theory_swiss_army_knife::conversion::interval;
use music_theory_swiss_army_knife::theory_primitive::interval::Interval;
use music_theory_swiss_army_knife::theory_primitive::note::Note;

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
fn get_a_flat_major_scale_should_return_a_flat_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::AFlat, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["A♭", "B♭", "C", "D♭", "E♭", "F", "G"];

    assert_eq!(actual, expected);
}

#[test]
fn get_a_major_scale_should_return_a_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::A, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["A", "B", "C♯", "D", "E", "F♯", "G♯"];

    assert_eq!(actual, expected);
}

#[test]
fn get_b_flat_major_scale_should_return_b_flat_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::BFlat, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["B♭", "C", "D", "E♭", "F", "G", "A"];

    assert_eq!(actual, expected);
}

#[test]
fn get_b_major_scale_should_return_b_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::B, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["B", "C♯", "D♯", "E", "F♯", "G♯", "A♯"];

    assert_eq!(actual, expected);
}

#[test]
fn get_c_flat_major_scale_should_return_c_flat_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::CFlat, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["C♭", "D♭", "E♭", "F♭", "G♭", "A♭", "B♭"];

    assert_eq!(actual, expected);
}

#[test]
fn get_c_major_scale_should_return_c_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::C, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["C", "D", "E", "F", "G", "A", "B"];

    assert_eq!(actual, expected);
}

#[test]
fn get_c_sharp_major_scale_should_return_c_sharp_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::CSharp, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["C♯", "D♯", "E♯", "F♯", "G♯", "A♯", "B♯"];

    assert_eq!(actual, expected);
}

#[test]
fn get_d_flat_major_scale_should_return_d_flat_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::DFlat, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["D♭", "E♭", "F", "G♭", "A♭", "B♭", "C"];

    assert_eq!(actual, expected);
}

#[test]
fn get_d_major_scale_should_return_d_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::D, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["D", "E", "F♯", "G", "A", "B", "C♯"];

    assert_eq!(actual, expected);
}

#[test]
fn get_e_flat_major_scale_should_return_e_flat_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::EFlat, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["E♭", "F", "G", "A♭", "B♭", "C", "D"];

    assert_eq!(actual, expected);
}

#[test]
fn get_e_major_scale_should_return_e_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::E, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["E", "F♯", "G♯", "A", "B", "C♯", "D♯"];

    assert_eq!(actual, expected);
}

#[test]
fn get_f_major_scale_should_return_f_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::F, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["F", "G", "A", "B♭", "C", "D", "E"];

    assert_eq!(actual, expected);
}

#[test]
fn get_f_sharp_major_scale_should_return_f_sharp_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::FSharp, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["F♯", "G♯", "A♯", "B", "C♯", "D♯", "E♯"];

    assert_eq!(actual, expected);
}

#[test]
fn get_g_flat_major_scale_should_return_g_flat_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::GFlat, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["G♭", "A♭", "B♭", "C♭", "D♭", "E♭", "F"];

    assert_eq!(actual, expected);
}

#[test]
fn get_g_major_scale_should_return_g_major_scale() {
    let actual = interval::get_notes_for_intervals(&Note::G, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["G", "A", "B", "C", "D", "E", "F♯"];

    assert_eq!(actual, expected);
}

#[test]
fn get_non_existent_f_flat_major_scale_should_return_f_flat_major_scale_with_double_flats() {
    let actual = interval::get_notes_for_intervals(&Note::FFlat, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["F♭", "G♭", "A♭", "B𝄫", "C♭", "D♭", "E♭"];

    assert_eq!(actual, expected);
}

#[test]
fn get_non_existent_g_sharp_major_scale_should_return_g_major_scale_with_double_sharps() {
    let actual = interval::get_notes_for_intervals(&Note::GSharp, &MAJOR_SCALE_INTERVALS.to_vec());

    let expected = vec!["G♯", "A♯", "B♯", "C♯", "D♯", "E♯", "F𝄪"];

    assert_eq!(actual, expected);
}
