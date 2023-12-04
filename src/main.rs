use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut content = File::open("assets/puzzle_input.txt").expect("couldn't find file");
    let mut buffer = String::new();

    content.read_to_string(&mut buffer)?;

    let lines = buffer.split("\n").collect::<Vec<_>>();

    let just_numbers = lines
        .iter()
        .map(|line| {
            line.to_lowercase()
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter(|char| "123456789".contains(*char))
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut awnser = 0;

    for numbers in just_numbers {
        let sum = match numbers.len() {
            0 => {
                continue;
            }
            1 => numbers[0] * 11,
            2 => (numbers[0] * 10) + numbers[1],
            _ => (numbers[0] * 10) + numbers[numbers.len() - 1],
        };
        // println!("{sum}");
        awnser += sum;
    }

    println!("the anwser = {:?}", awnser);

    Ok(())
}
