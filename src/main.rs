mod lib;
fn main() {
    let contents = lib::contents("pgn.txt");
    println!("{}", lib::clean(contents));
}
