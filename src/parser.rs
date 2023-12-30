use std::fs;
pub fn parse_content(file: &str) -> String {
    let path = String::from("assets/");
    let extension = ".txt";
    let file = path + file + extension;
    let mut content = String::new();
    if let Ok(output) = fs::read_to_string(file) {
        content = output;
    }
    content
}
