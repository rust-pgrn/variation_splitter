mod lib;
use std::fs;
fn main() {
    let contents = lib::contents("don-t_change.txt");
    let contents = lib::clean(contents);
    let variations = lib::split(contents, "pgn.txt");
    fs::write("file.pgn", lib::convert(&variations)).unwrap();
}
