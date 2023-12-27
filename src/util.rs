use crate::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StarMap {
    Star(Point),
    None,
}

pub fn parse_char_to_galaxy_map(c: char, x: usize, y: usize) -> StarMap {
    match c {
        '.' => StarMap::None,
        '#' => StarMap::Star(Point::new(x, y)),
        _ => unreachable!(),
    }
}
