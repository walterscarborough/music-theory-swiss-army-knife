use music_theory_swiss_army_knife::interval::Interval;
use music_theory_swiss_army_knife::scale;
use music_theory_swiss_army_knife::scale::Scale;

#[test]
fn get_c_major_scale_should_return_c_major_scale() {
    let major_scale_intervals = vec![
        Interval::Perfect1,
        Interval::Major2,
        Interval::Major3,
        Interval::Perfect4,
        Interval::Perfect5,
        Interval::Major6,
        Interval::Major7,
    ];

    let actual = scale::get_notes_for_scale_degrees("C", &major_scale_intervals);

    let expected = vec!["C", "D", "E", "F", "G", "A", "B"];

    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn get_d_major_scale_should_return_c_major_scale() {
    let major_scale_intervals = vec![
        Interval::Perfect1,
        Interval::Major2,
        Interval::Major3,
        Interval::Perfect4,
        Interval::Perfect5,
        Interval::Major6,
        Interval::Major7,
    ];

    let actual = scale::get_notes_for_scale_degrees("D", &major_scale_intervals);

    let expected = vec!["D", "E", "F♯", "G", "A", "B", "C♯"];

    assert_eq!(
        actual,
        expected
    );
}
