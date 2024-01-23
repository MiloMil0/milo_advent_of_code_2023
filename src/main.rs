use parser::parse_content;

mod parser;

fn is_reflection(reflection: &str) -> bool {
    let reverse = reflection.chars().rev().collect::<String>();
    reflection == reverse && reflection.len() % 2 == 0
}

fn is_valid_reflection(pattern: &Vec<&str>, index: usize, length: usize) -> bool {
    pattern.iter().all(|row| {
        let current = &row[index..(index + length)];
        is_reflection(current)
    })
}

fn validate_mirror(pattern: &str) -> Option<(usize, usize)> {
    let pattern = pattern.lines().collect::<Vec<&str>>();

    if let Some(&first_row) = pattern.first() {
        for index in 0..first_row.len() {
            let mirror = &first_row[index..first_row.len()];
            if mirror.len() > 1 {
                if is_valid_reflection(&pattern, index, mirror.len()) {
                    return Some((index, mirror.len()));
                }
            }
        }

        for index in (0..first_row.len()).rev() {
            let mirror = &first_row[0..index];
            if mirror.len() > 1 {
                if is_valid_reflection(&pattern, 0, mirror.len()) {
                    return Some((0, mirror.len()));
                }
            }
        }
    }
    None
}

fn rotate_map(input: &str) -> String {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let rotated_matrix: Vec<Vec<char>> = (0..cols)
        .map(|j| (0..rows).map(|i| matrix[i][j]).collect())
        .collect();

    let rotated_string: String = rotated_matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    rotated_string
}

fn main() {
    let content = parse_content("puzzle_input");

    let patterns = content.trim().split("\n\n").collect::<Vec<&str>>();

    let mut answer = 0;

    for pattern in patterns.iter() {
        if let Some((index, size)) = validate_mirror(pattern) {
            answer += (size + 1) / 2 + index;
        } else if let Some((index, size)) = validate_mirror(&rotate_map(pattern)) {
            answer += ((size + 1) / 2 + index) * 100;
        }
    }

    println!("the answer = {answer}");
}
