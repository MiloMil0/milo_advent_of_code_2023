pub const VALID_UP: [Piece; 4] = [
    Piece::Vertical,
    Piece::TopLeftCorner,
    Piece::TopRightCorner,
    Piece::Animal,
];

pub const VALID_DOWN: [Piece; 4] = [
    Piece::Vertical,
    Piece::BottomLeftCorner,
    Piece::BottomRightCorner,
    Piece::Animal,
];

pub const VALID_LEFT: [Piece; 4] = [
    Piece::Horizontal,
    Piece::BottomLeftCorner,
    Piece::TopLeftCorner,
    Piece::Animal,
];

pub const VALID_RIGHT: [Piece; 4] = [
    Piece::Horizontal,
    Piece::TopRightCorner,
    Piece::BottomRightCorner,
    Piece::Animal,
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    Vertical,
    Horizontal,
    BottomLeftCorner,
    BottomRightCorner,
    TopLeftCorner,
    TopRightCorner,
    Ground,
    Animal,
}

pub fn parse_char_to_piece(c: char) -> Piece {
    match c {
        '.' => Piece::Ground,
        '|' => Piece::Vertical,
        '-' => Piece::Horizontal,
        'L' => Piece::BottomLeftCorner,
        'J' => Piece::BottomRightCorner,
        '7' => Piece::TopRightCorner,
        'F' => Piece::TopLeftCorner,
        'S' => Piece::Animal,

        _ => unreachable!(),
    }
}

pub fn is_valid_up(origin_piece: &Piece, target_piece: &Piece) -> bool {
    match origin_piece {
        Piece::Animal => VALID_UP.contains(target_piece),
        Piece::Vertical => VALID_UP.contains(target_piece),
        Piece::BottomLeftCorner => VALID_UP.contains(target_piece),
        Piece::BottomRightCorner => VALID_UP.contains(target_piece),
        _ => false,
    }
}

pub fn is_valid_down(origin_piece: &Piece, target_piece: &Piece) -> bool {
    match origin_piece {
        Piece::Animal => VALID_DOWN.contains(target_piece),
        Piece::Vertical => VALID_DOWN.contains(target_piece),
        Piece::TopLeftCorner => VALID_DOWN.contains(target_piece),
        Piece::TopRightCorner => VALID_DOWN.contains(target_piece),
        _ => false,
    }
}

pub fn is_valid_left(origin_piece: &Piece, target_piece: &Piece) -> bool {
    match origin_piece {
        Piece::Animal => VALID_LEFT.contains(target_piece),
        Piece::Horizontal => VALID_LEFT.contains(target_piece),
        Piece::TopRightCorner => VALID_LEFT.contains(target_piece),
        Piece::BottomRightCorner => VALID_LEFT.contains(target_piece),
        _ => false,
    }
}

pub fn is_valid_right(origin_piece: &Piece, target_piece: &Piece) -> bool {
    match origin_piece {
        Piece::Animal => VALID_RIGHT.contains(target_piece),
        Piece::Horizontal => VALID_RIGHT.contains(target_piece),
        Piece::BottomLeftCorner => VALID_RIGHT.contains(target_piece),
        Piece::TopLeftCorner => VALID_RIGHT.contains(target_piece),
        _ => false,
    }
}
