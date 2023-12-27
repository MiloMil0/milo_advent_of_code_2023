use std::{collections::HashMap, vec};

use crate::{
    point::Point,
    util::{parse_char_to_galaxy_map, StarMap},
};

#[derive(Debug)]
pub struct Map {
    galaxy_coords: Vec<Point>,
}

impl Map {
    pub fn new(content: String) -> Self {
        let mut galaxy_map = vec![];
        let mut galaxy_coords = Vec::new();
        let mut rotated_map = HashMap::new();
        let x_multiplier: usize = 1000000;
        let y_multiplier: usize = 1000000;
        let mut x_distance: usize = 0;
        let mut y_distance: usize = 0;

        for (y, line) in content.lines().enumerate() {
            let mut piece_per_row = Vec::new();

            line.chars().enumerate().for_each(|(x, c)| {
                let star_map_part = parse_char_to_galaxy_map(c, x, y + y_distance);
                piece_per_row.push(star_map_part.clone());
                rotated_map
                    .entry(x)
                    .or_insert_with(Vec::new)
                    .push(star_map_part.clone());
            });
            if piece_per_row.iter().all(|&g| g == StarMap::None) {
                y_distance += y_multiplier - 1;
            }
            galaxy_map.push(piece_per_row);
        }

        let map_height = galaxy_map.len();

        for key in 0..map_height {
            if let Some(vec) = rotated_map.get_mut(&key) {
                for star in vec
                    .iter_mut()
                    .filter(|star| matches!(star, StarMap::Star(_)))
                {
                    if let StarMap::Star(point) = star {
                        point.x += x_distance as isize;
                    }
                }
                if vec.iter().all(|&g| g == StarMap::None) {
                    x_distance += x_multiplier - 1;
                }
            }
        }

        for (_, vec) in rotated_map.iter() {
            for star in vec.iter() {
                if let StarMap::Star(point) = star {
                    galaxy_coords.push(*point);
                }
            }
        }

        let map_width = galaxy_map[0].len();
        galaxy_coords.sort();

        println!("map = {map_height} x {map_width}");

        Map { galaxy_coords }
    }

    pub fn calculate_paths_between_galaxies(&self) -> isize {
        let mut sum = 0;
        for galaxy in 0..self.galaxy_coords.len() {
            for other_galaxy in galaxy + 1..self.galaxy_coords.len() {
                sum += self.galaxy_coords[galaxy].manhattan(self.galaxy_coords[other_galaxy]);
            }
        }

        sum
    }
}
