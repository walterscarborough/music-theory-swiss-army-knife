use music_theory_swiss_army_knife::interval::Interval;
use music_theory_swiss_army_knife::scale;
use music_theory_swiss_army_knife::scale::Scale;

#[test]
fn get_scale_should_return_a_scale_when_it_exists() {
    let actual = scale::get("major");

    let expected = Scale {
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
    };

    assert_eq!(expected, actual)
}

#[test]
#[should_panic(expected = "Unable to find the scale: \"yolo\"")]
fn get_scale_should_panic_when_a_scale_does_not_exist() {
    scale::get("yolo");
}
