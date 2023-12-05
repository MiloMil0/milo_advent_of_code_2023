use std::fs::File;
use std::io::prelude::*;

use vector::Point;

use crate::util::{parse_char, query_surrounding_tiles};

mod util;
mod vector;

#[derive(Debug)]
struct Map {
    tile_type: Vec<TileType>,
    tile_coord: Vec<Point>,
    map_width: usize,
    map_height: usize,
}

impl Map {
    fn new(
        tile_type: Vec<TileType>,
        tile_coord: Vec<Point>,
        map_width: usize,
        map_height: usize,
    ) -> Self {
        Map {
            tile_type,
            tile_coord,
            map_width,
            map_height,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum TileType {
    Number(char),
    Symbol,
    Blank,
}

fn main() -> std::io::Result<()> {
    let mut content = File::open("assets/puzzle_input.txt").expect("couldn't find file");
    let mut buffer = String::new();

    content.read_to_string(&mut buffer)?;

    let lines = buffer.trim().split("\n").collect::<Vec<_>>();

    let mut tile_type = Vec::new();
    let mut tile_coords = Vec::new();

    let map_width = lines[0].len();
    let map_height = lines.len();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = parse_char(c);
            let coords = Point::new(x, y);

            tile_type.push(tile);
            tile_coords.push(coords);
        }
    }

    let map = Map::new(tile_type, tile_coords, map_width, map_height);
    let mut sum = 0;

    let mut memory = String::new();
    let mut part_has_symbol = false;

    for (id, tiles) in map.tile_coord.iter().enumerate() {
        if let TileType::Number(value) = map.tile_type[id] {
            memory.push(value);
            if query_surrounding_tiles(&map, tiles) {
                part_has_symbol = true;
            }
        }
        if map.tile_type[id] == TileType::Blank && part_has_symbol
            || map.tile_type[id] == TileType::Symbol && part_has_symbol
        {
            if let Ok(number) = memory.parse::<i32>() {
                sum += number;
                memory.clear();
            }
            part_has_symbol = false;
        } else if map.tile_type[id] == TileType::Blank && !part_has_symbol {
            memory.clear();
        }
    }

    println!("{sum}");

    Ok(())
}
