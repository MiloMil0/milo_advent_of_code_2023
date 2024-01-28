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
    let mut one_letter_difference_count = 0;

    for row in pattern.iter() {
        let current = &row[index..(index + length)].bytes().collect::<Vec<_>>();
        // let reversed: String = current.chars().rev().collect();

        // println!("{}", current);
        // println!("{}", reversed);

        for i in 0..current.len() {
            for j in i + 1..current.len() {
                if current[i].zip(current[j]).filter(|(a, b)| a != b).count() == 1 {
                    // if current
                    //     .chars()
                    //     .zip(reversed.chars())
                    //     .filter(|(a, b)| a != b)
                    //     .count()
                    //     == 1
                    // {
                    one_letter_difference_count += 1;
                    println!("{one_letter_difference_count}");
                } else {
                    println!();
                    return false;
                }
            }
        }
    }
    println!();

    one_letter_difference_count == 1
}

fn is_valid_reflection(pattern: &str) -> bool {
    // pattern.iter().all(|row| {
    // let reflection = &row[index..=(index + length)];
    let reverse = pattern.chars().rev().collect::<String>();
    pattern == reverse && pattern.len() % 2 == 0
    // })
}

// fn find_smudge(pattern: &str) -> Option<(usize, usize)> {
//     let mut lines = pattern.lines().collect::<Vec<&str>>();
//
//     let line_count = lines.len();
//
//     for i in 0..line_count {
//         for j in i + 1..line_count {
//             if lines[i]
//                 .chars()
//                 .zip(lines[j].chars())
//                 .filter(|(a, b)| a != b)
//                 .count()
//                 == 1
//             {
//                 println!(
//                     "line {i} {} is different from line {j} {} and line length = {}",
//                     lines[i],
//                     lines[j],
//                     lines.len(),
//                 );
//                 let mem = lines.clone();
//                 lines[i] = lines[j];
//
//                 if is_valid_smudge(&rotate_map(&lines.join("\n")), i, j) {
//                     return Some((i, j - i));
//                 } else {
//                     lines = mem;
//                 }
//             }
//         }
//     }
//
//     None
// }

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
    let content = parse_content("test_input");

    let patterns = content.trim().split("\n\n").collect::<Vec<&str>>();

    let mut answer = 0;

    for (i, pattern) in patterns.iter().enumerate() {
        println!("pattern {}: ", i + 1);
        println!("checking horizontal pattern");

        if let Some((index, size)) = valid_mirror(pattern) {
            answer += calc_value(index, size) * 100;
            println!(
                "map is horizontal for a value of {}",
                calc_value(index, size) * 100
            );
            println!();
            continue;
        } else {
            println!("checking vertical pattern")
        }
        if let Some((index, size)) = valid_mirror(&rotate_map(pattern)) {
            answer += calc_value(index, size);
            println!("map is vertical for a value of {}", calc_value(index, size))
        }
        println!();
    }

    println!("the answer = {answer}");
}
