use music_theory_swiss_army_knife::interval::Interval;
use music_theory_swiss_army_knife::scale;
use music_theory_swiss_army_knife::scale::Scale;

#[test]
fn get_all_scales_should_return_all_scales() {
    let actual = scale::get_all();

    assert_eq!(
        actual.iter().any(|scale| scale.name == "major".to_string()),
        true,
    );

    assert!(actual.len() > 1);
}

#[test]
fn get_scale_should_return_a_major_scale() {
    let actual = scale::get("major");

    let expected = Scale {
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
    };

    assert_eq!(expected, actual)
}

#[test]
fn get_scale_should_return_a_minor_scale() {
    let actual = scale::get("minor");

    let expected = Scale {
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
    };

    assert_eq!(expected, actual)
}

#[test]
#[should_panic(expected = "Unable to find the scale: \"yolo\"")]
fn get_scale_should_panic_when_a_scale_does_not_exist() {
    scale::get("yolo");
}
