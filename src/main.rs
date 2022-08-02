mod lib;
use std::fs;
fn main() {
    let contents = lib::contents("files/input.txt");
    let contents = lib::clean(contents);
    let variations = lib::split(contents, "files/logs.txt");
    fs::write("files/output.pgn", lib::convert(&variations)).unwrap();
}
