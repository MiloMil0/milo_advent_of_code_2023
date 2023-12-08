use std::fs;

fn main() {
    //my template for importing the advent puzzles :)
    let content = match fs::read_to_string("assets/test_input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
}
