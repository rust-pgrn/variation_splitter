mod lib;
use std::fs;
fn main() {
    let contents = lib::contents("input.txt");
    let contents = lib::clean(contents);
    let variations = lib::split(contents, "logs.txt");
    fs::write("output.pgn", lib::convert(&variations)).unwrap();
}
