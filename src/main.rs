use std::fs;
mod util;
use util::*;

fn main() {
    let content = match fs::read_to_string("assets/puzzle_input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut numbers = Vec::new();

    for lines in content.lines() {
        let parsed_numbers = lines
            .split(" ")
            .map(|string| string.parse::<i32>().unwrap_or_default())
            .collect::<Vec<_>>();
        numbers.push(parsed_numbers);
    }

    let mut answer = 0;

    for sequence in numbers.iter_mut() {
        sequence.reverse();
        let differences = calculate_differences(&sequence);
        let result = get_final_number(&differences);
        answer += result;
    }

    println!("the answer = {answer}");
}
