mod lib;
fn main() {
    let contents = lib::contents("pgn.txt");
    let contents = lib::clean(contents);
    lib::split(contents);
}
