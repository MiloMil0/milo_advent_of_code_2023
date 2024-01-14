#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

pub fn parse_char_to_condition(c: char) -> SpringCondition {
    match c {
        '.' => SpringCondition::Operational,
        '#' => SpringCondition::Damaged,
        '?' => SpringCondition::Unknown,
        _ => unreachable!(),
    }
}
