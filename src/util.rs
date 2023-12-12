#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub position: String,
    pub left: String,
    pub right: String,
}

pub fn parse_direction(direction: &str) -> Vec<Direction> {
    direction
        .chars()
        .filter_map(|c| match c {
            'R' => Some(Direction::Right),
            'L' => Some(Direction::Left),
            _ => None,
        })
        .collect()
}
