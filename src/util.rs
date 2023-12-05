use crate::{vector::Point, Map, TileType};

pub fn parse_char(c: char) -> TileType {
    match c {
        '0'..='9' => TileType::Number(c),
        '.' => TileType::Blank,
        _ => TileType::Symbol,
    }
}

pub fn xy_to_id(x: i32, y: i32, map_width: usize) -> usize {
    (y * map_width as i32 + x) as usize
}

pub fn query_surrounding_tiles(map: &Map, point: &Point) -> bool {
    let check_tiles = vec![
        Point::UP,
        Point::DOWN,
        Point::LEFT,
        Point::RIGHT,
        Point::UP_LEFT,
        Point::UP_RIGHT,
        Point::DOWN_LEFT,
        Point::DOWN_RIGHT,
    ];

    for tile in check_tiles {
        let x = point.x + tile.x;
        let y = point.y + tile.y;
        if (x as usize) < map.map_width && (y as usize) < map.map_height {
            let id = xy_to_id(x, y, map.map_width);
            if let TileType::Symbol = map.tile_type[id] {
                return true;
            }
        }
    }

    false
}
