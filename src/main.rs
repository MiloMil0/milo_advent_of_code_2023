use parser::parse_content;

mod parser;

fn valid_mirror(pattern: &str) -> Option<(usize, usize)> {
    let lines = pattern.lines().collect::<Vec<&str>>();

    if let Some(&first_row) = lines.first() {
        for index in 0..first_row.len() {
            let forward_mirror = &first_row[index..first_row.len()];
            let backward_mirror = &first_row[0..(first_row.len() - index)];

            if forward_mirror.len() > 1 && is_valid_smudge(&lines, index, forward_mirror.len()) {
                return Some((index, forward_mirror.len()));
            } else if backward_mirror.len() > 1 && is_valid_smudge(&lines, 0, backward_mirror.len())
            {
                return Some((0, backward_mirror.len()));
            }
        }
    }
    None
}

fn is_valid_smudge(pattern: &Vec<&str>, index: usize, length: usize) -> bool {
    let mut smudge_count = 0;

    for row in pattern.iter() {
        let current = &row[index..(index + length)];
        if current.len() % 2 != 0 {
            return false;
        }

        let (first_half, second_half) = current.split_at(current.len() / 2);

        smudge_count += first_half
            .chars()
            .zip(second_half.chars().rev())
            .filter(|(a, b)| a != b)
            .count();
        if smudge_count > 1 {
            return false;
        }
    }

    smudge_count == 1
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

fn calc_value(index: usize, size: usize) -> usize {
    (size + 1) / 2 + index
}

fn main() {
    let content = parse_content("puzzle_input");

    let patterns = content.trim().split("\n\n").collect::<Vec<&str>>();

    let mut answer = 0;

    for pattern in patterns.iter() {
        if let Some((index, size)) = valid_mirror(pattern) {
            answer += calc_value(index, size);
        } else if let Some((index, size)) = valid_mirror(&rotate_map(pattern)) {
            answer += calc_value(index, size) * 100;
        }
    }

    println!("the answer = {answer}");
}
