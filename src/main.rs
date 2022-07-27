mod lib;
fn main() {
    let contents = lib::contents("don-t_change.txt");
    let contents = lib::clean(contents);
    lib::split(contents, "pgn.txt");
}
