use crate::{vector::Point, Map, TileType};

pub fn parse_char(c: char) -> TileType {
    match c {
        '0'..='9' => TileType::Number(c),
        '.' => TileType::Blank,
        '*' => TileType::Gear,
        _ => TileType::Symbol,
    }
}

pub fn xy_to_id(x: i32, y: i32, map_width: usize) -> usize {
    (y * map_width as i32 + x) as usize
}

pub fn scannable_tiles() -> Vec<Point> {
    vec![
        Point::UP,
        Point::DOWN,
        Point::LEFT,
        Point::RIGHT,
        Point::UP_LEFT,
        Point::UP_RIGHT,
        Point::DOWN_LEFT,
        Point::DOWN_RIGHT,
    ]
}

pub fn check_for_gears(map: &Map, id: usize) -> Option<usize> {
    let check_tiles = scannable_tiles();
    let point = map.tile_coord[id];
    for tile in check_tiles {
        let x = point.x + tile.x;
        let y = point.y + tile.y;
        if (x as usize) < map.map_width && (y as usize) < map.map_height {
            let gear_id = xy_to_id(x, y, map.map_width);
            if map.tile_type[gear_id] == TileType::Gear {
                return Some(gear_id);
            }
        }
    }

    None
}
