use crate::theory_primitive::interval::Interval;

#[derive(PartialEq, Debug, Clone)]
pub struct Scale {
    pub name: String,
    pub aliases: Vec<String>,
    pub intervals: Vec<Interval>,
}
