use std::collections::{HashSet, VecDeque};

use crate::{
    point::{Point, ORTHO_DIRECTIONS},
    util::*,
};

#[derive(Debug)]
pub struct Map {
    pub map_width: usize,
    pub map_height: usize,
    pub map_pieces: Vec<Vec<Piece>>,
    pub map_coords: Vec<Point>,
    pub full_loop: Vec<Point>,
}

impl Map {
    pub fn new(content: String) -> Self {
        let mut map_pieces = vec![];
        let mut map_coords = Vec::new();

        for (y, line) in content.lines().enumerate() {
            let mut piece_per_line = Vec::new();

            line.chars().enumerate().for_each(|(x, c)| {
                map_coords.push(Point::new(x, y));
                piece_per_line.push(parse_char_to_piece(c));
            });
            map_pieces.push(piece_per_line);
        }

        let map_height = map_pieces.len();
        let map_width = map_pieces[0].len();
        println!("map = {map_height} x {map_width}");

        let full_loop = Vec::new();

        Map {
            map_width,
            map_height,
            map_pieces,
            map_coords,
            full_loop,
        }
    }

    pub fn find_animal(&self) -> Option<usize> {
        self.map_pieces
            .iter()
            .flatten()
            .position(|piece| piece == &Piece::Animal)
    }

    pub fn try_move(&mut self) -> usize {
        let mut count = 0;
        if let Some(animal) = self.find_animal() {
            let start_point = self.map_coords[animal];

            let mut visisted = HashSet::new();
            let mut queue = VecDeque::new();

            queue.push_back((start_point, count));
            visisted.insert(start_point);

            while let Some((pos, step)) = queue.pop_front() {
                count = step;

                for neighbour in self.get_neighbours(pos) {
                    if !visisted.contains(&neighbour) {
                        visisted.insert(neighbour);
                        queue.push_back((neighbour, step + 1));
                    }
                }
            }

            for points in visisted {
                self.full_loop.push(points);
            }
        }
        count
    }

    fn get_neighbours(&self, origin: Point) -> Vec<Point> {
        let mut neighbours = Vec::new();

        for direction in ORTHO_DIRECTIONS {
            let target = origin + direction;

            if self.is_in_bounds(&target) && self.is_valid_move(origin, &target) {
                neighbours.push(target);
            }
        }

        neighbours
    }

    fn is_in_bounds(&self, target: &Point) -> bool {
        target.x >= 0
            && target.x < self.map_width as i32
            && target.y >= 0
            && target.y < self.map_height as i32
    }

    fn is_valid_move(&self, origin: Point, target: &Point) -> bool {
        let origin_piece = self.map_pieces[origin.y as usize][origin.x as usize];
        let target_piece = self.map_pieces[target.y as usize][target.x as usize];
        let difference = *target - origin;
        match difference {
            Point::UP => is_valid_up(&origin_piece, &target_piece),
            Point::DOWN => is_valid_down(&origin_piece, &target_piece),
            Point::LEFT => is_valid_left(&origin_piece, &target_piece),
            Point::RIGHT => is_valid_right(&origin_piece, &target_piece),
            _ => false,
        }
    }
}
