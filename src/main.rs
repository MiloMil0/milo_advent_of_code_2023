use std::io::prelude::*;
use std::{collections::HashMap, fs::File};

use vector::Point;

use crate::util::{check_for_gears, parse_char};

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
    Gear,
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

    let mut number_as_string = String::new();
    let mut number_storage = HashMap::new();

    let mut has_gear = false;
    let mut gear_id = 0;

    for (id, tile) in map.tile_type.iter().enumerate() {
        match *tile {
            TileType::Number(value) => {
                number_as_string.push(value);
                if let Some(gear) = check_for_gears(&map, id) {
                    gear_id = gear;
                    has_gear = true;
                }
            }
            _ => {
                if has_gear {
                    let number = number_as_string.parse::<i32>().unwrap();
                    number_storage
                        .entry(gear_id)
                        .or_insert(Vec::new())
                        .push(number);
                }
                number_as_string.clear();
                has_gear = false;
            }
        }
    }

    let mut sum = 0;

    for (_, value) in &mut number_storage {
        if value.len() > 1 {
            sum += value.iter().product::<i32>();
        }
    }

    println!("the awnser = {sum}");

    Ok(())
}
