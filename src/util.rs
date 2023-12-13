#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub position: Vec<char>,
    pub left: Vec<char>,
    pub right: Vec<char>,
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

pub fn find_path_length(
    position: &Location,
    directions: &Vec<Direction>,
    locations: &Vec<Location>,
) -> Option<usize> {
    let mut current_position = &position.position;
    let mut count = 0;
    loop {
        for &direction in directions {
            count += 1;
            let find_location = locations
                .iter()
                .find(|x| x.position == *current_position)
                .map(|x| {
                    if direction == Direction::Left {
                        &x.left
                    } else {
                        &x.right
                    }
                });
            if let Some(next_location) = find_location {
                current_position = next_location;
                if current_position.ends_with(&['Z']) {
                    return Some(count);
                }
            }
        }
    }
}

pub fn calculate_lcm(numbers: &[usize]) -> usize {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];
    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }
    result
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}
