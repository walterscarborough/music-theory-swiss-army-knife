use music_theory_swiss_army_knife::conversion::interval;
use music_theory_swiss_army_knife::theory_primitive::interval::Interval;
use music_theory_swiss_army_knife::theory_primitive::note::Note;

#[test]
fn convert_perfect1() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Perfect1]);

    let expected = vec!["C"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_minor2() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Minor2]);

    let expected = vec!["D♭"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_major2() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Major2]);

    let expected = vec!["D"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_minor3() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Minor3]);

    let expected = vec!["E♭"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_major3() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Major3]);

    let expected = vec!["E"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_perfect4() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Perfect4]);

    let expected = vec!["F"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_perfect5() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Perfect5]);

    let expected = vec!["G"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_minor6() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Minor6]);

    let expected = vec!["A♭"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_major6() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Major6]);

    let expected = vec!["A"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_minor7() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Minor7]);

    let expected = vec!["B♭"];

    assert_eq!(actual, expected);
}

#[test]
fn convert_major7() {
    let actual = interval::get_notes_for_intervals(&Note::C, &vec![Interval::Major7]);

    let expected = vec!["B"];

    assert_eq!(actual, expected);
}

#[test]
fn double_flats_supported() {
    let actual = interval::get_notes_for_intervals(&Note::FFlat, &vec![Interval::Perfect4]);

    let expected = vec!["B𝄫"];

    assert_eq!(actual, expected);
}

#[test]
fn double_sharps_supported() {
    let actual = interval::get_notes_for_intervals(&Note::GSharp, &vec![Interval::Major7]);

    let expected = vec!["F𝄪"];

    assert_eq!(actual, expected);
}
