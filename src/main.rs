use std::fs;

fn main() {
    let content = match fs::read_to_string("assets/puzzle_input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut time = 0;
    let mut distances = 0;

    for line in content.lines() {
        if line.starts_with("Time:") {
            time = line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .replace(" ", "")
                .parse::<usize>()
                .unwrap();
        }
        if line.starts_with("Distance") {
            distances = line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .replace(" ", "")
                .parse::<usize>()
                .unwrap();
        }
    }

    let mut answer_part2 = 0;

    for x in 0..=time {
        let speed = x;
        let distance = (time - x) * speed;
        if distance > distances {
            answer_part2 += 1
        }
    }

    println!("the answer = {answer_part2}");
}
