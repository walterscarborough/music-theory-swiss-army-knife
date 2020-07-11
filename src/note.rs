use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Hash, Eq, Copy)]
pub enum Note {
    AFlatFlat,
    AFlat,
    A,
    ASharp,
    ASharpSharp,

    BFlatFlat,
    BFlat,
    B,
    BSharp,
    BSharpSharp,

    CFlatFlat,
    CFlat,
    C,
    CSharp,
    CSharpSharp,

    DFlatFlat,
    DFlat,
    D,
    DSharp,
    DSharpSharp,

    EFlatFlat,
    EFlat,
    E,
    ESharp,
    ESharpSharp,

    FFlatFlat,
    FFlat,
    F,
    FSharp,
    FSharpSharp,

    GFlatFlat,
    GFlat,
    G,
    GSharp,
    GSharpSharp,
}

pub fn note_to_string(note: &Note) -> &'static str {
    let note_mappings: HashMap<Note, &str> = [
        (Note::AFlatFlat, "A𝄫"),
        (Note::AFlat, "A♭"),
        (Note::A, "A"),
        (Note::ASharp, "A♯"),
        (Note::ASharpSharp, "A𝄪"),
        (Note::BFlatFlat, "B𝄫"),
        (Note::BFlat, "B♭"),
        (Note::B, "B"),
        (Note::BSharp, "B♯"),
        (Note::BSharpSharp, "B𝄪"),
        (Note::CFlatFlat, "C𝄫"),
        (Note::CFlat, "C♭"),
        (Note::C, "C"),
        (Note::CSharp, "C♯"),
        (Note::CSharpSharp, "C𝄪"),
        (Note::DFlatFlat, "D𝄫"),
        (Note::DFlat, "D♭"),
        (Note::D, "D"),
        (Note::DSharp, "D♯"),
        (Note::DSharpSharp, "D𝄪"),
        (Note::EFlatFlat, "E𝄫"),
        (Note::EFlat, "E♭"),
        (Note::E, "E"),
        (Note::ESharp, "E♯"),
        (Note::ESharpSharp, "E𝄪"),
        (Note::FFlatFlat, "F𝄫"),
        (Note::FFlat, "F♭"),
        (Note::F, "F"),
        (Note::FSharp, "F♯"),
        (Note::FSharpSharp, "F𝄪"),
        (Note::GFlatFlat, "G𝄫"),
        (Note::GFlat, "G♭"),
        (Note::G, "G"),
        (Note::GSharp, "G♯"),
        (Note::GSharpSharp, "G𝄪"),
    ].iter().cloned().collect();

    note_mappings.get(note).unwrap()
}
