use std::fs;
use std::io::{BufReader, Write};
use std::path::PathBuf;

pub fn parse_markdown(path: PathBuf) -> String {
    let contents = fs::read_to_string(path).expect("Failed to read markdown.");
    markdown::to_html(&contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown() {
        let path = PathBuf::from("./notes/test.md");
        let content = parse_markdown(path);
        let expected = String::from(
            "<h1>Test Note</h1>\n<h2>Hello world!</h2>\n<p>This is the test page.</p>",
        );
        assert_eq!(content, expected);
    }
}

fn main() {
    // Transform markdown to htmls
    // fs::create_dir("asssets").unwrap_or_else(|_| println!("File already exists"));
    let path = PathBuf::from("notes/test.md");
    let contents = parse_markdown(path);
    let mut file = fs::File::create("assets/test.html").expect("Failed to create a file");
    write!(file, "{}", &contents).unwrap();
}
