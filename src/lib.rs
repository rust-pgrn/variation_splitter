use std::env;
use std::fs;
pub enum Color {
    White,
    Black,
}
pub enum Order {
    Main,
    Side,
}
pub struct Config<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
    pub logs_file: &'a str,
    pub color: Color,
    pub main_line: Order,
}
impl<'a> Config<'a> {
    pub fn new(mut args: env::Args) -> Result<Config<'a>, &'a str> {
        let input_file = "files/input.txt";
        let output_file = "files/output.pgn";
        let logs_file = "files/logs.txt";

        //First argument is path of executable
        args.next();
        let color = match args.next() {
            Some(arg) => {
                if arg == "w" || arg == "white" {
                    Color::White
                } else if arg == "b" || arg == "black" {
                    Color::Black
                } else {
                    return Err("Not a color");
                }
            }
            None => return Err("No color specified"),
        };

        let main_line = match args.next() {
            Some(arg) => {
                if arg == "m" || arg == "main" {
                    Order::Main
                } else if arg == "s" || arg == "side" {
                    Order::Side
                } else {
                    return Err("Not an order");
                }
            }
            None => return Err("No order specified"),
        };
        Ok(Config {
            input_file,
            output_file,
            logs_file,
            color,
            main_line,
        })
    }
}
pub fn contents(filename: &str) -> String {
    fs::read_to_string(filename).expect("File Not Found")
}
pub fn clean(contents: String) -> String {
    let mut s = String::new();
    for i in contents.chars() {
        if i == '\n' {
            s.push(' ');
        } else {
            s.push(i);
        }
    }
    s
}
pub fn split(contents: String, filename: &str) -> Vec<Vec<String>> {
    let mut variations: Vec<Vec<String>> = Vec::new();
    let mut i = variations.len();
    variations.push(Vec::new());
    let mut buf = String::new();
    for (len, pli) in contents.split(' ').enumerate() {
        if pli.contains('(') {
            variations.insert(i, variations[i].clone());
            i += 1;
            variations[i].pop();
            buf.push_str(&format!("Pli: {}\n", pli));
            buf.push_str("Action: New Variation\n");
            buf.push_str(&format!("Variation: {:?}\n", variations[i]));
            buf.push_str(&format!("Working on variation: {}\n", i + 1));
            buf.push_str(&format!("Number of variations: {}\n", variations.len()));
            buf.push_str(&format!("Pli Number: {}\n", len + 1));
            buf.push_str("\n\n");
        } else if pli.contains(')') {
            buf.push_str(&format!("Pli: {}\n", pli));
            if &pli[0..1] != ")" && &pli[0..1] != "$" && !pli.contains('{') && !pli.contains('}') {
                variations[i].push(pli.replace(')', ""));
                buf.push_str(&format!(
                    "Action:  Adding {} & Moving Back Variation\n",
                    pli
                ));
            } else {
                buf.push_str("Action:  Moving Back Variation\n");
            }
            buf.push_str(&format!("Variation: {:?}\n", variations[i]));
            buf.push_str(&format!("Working on variation: {}\n", i + 1));
            buf.push_str(&format!("Number of variations: {}\n", variations.len()));
            buf.push_str(&format!("Pli Number: {}\n", len + 1));
            for c in pli.chars() {
                if c == ')' {
                    i -= 1;
                }
            }
            buf.push_str(&format!("Now Working on variation: {}\n", i + 1));
            buf.push_str(&format!("Variation Now: {:?}\n", variations[i]));
            buf.push_str("\n\n");
        } else if pli.contains("...") | pli.contains('$') | pli.contains('{') | pli.contains('}') {
            buf.push_str(&format!("Pli: {}\n", pli));
            buf.push_str("Action: Nothing\n");
            buf.push_str(&format!("Variation: {:?}\n", variations[i]));
            buf.push_str(&format!("Working on variation: {}\n", i + 1));
            buf.push_str(&format!("Number of variations: {}\n", variations.len()));
            buf.push_str(&format!("Pli Number: {}\n", len + 1));
            buf.push_str("\n\n");
        } else {
            variations[i].push(pli.to_string());
            buf.push_str(&format!("Pli: {}\n", pli));
            buf.push_str(&format!("Action: Adding {}\n", pli));
            buf.push_str(&format!("Variation: {:?}\n", variations[i]));
            buf.push_str(&format!("Working on variation: {}\n", i + 1));
            buf.push_str(&format!("Number of variations: {}\n", variations.len()));
            buf.push_str(&format!("Pli Number: {}\n", len + 1));
            buf.push_str("\n\n");
        }
    }
    let _s = convert(&variations);
    fs::write(filename, buf).unwrap();
    variations
}
pub fn convert(v: &Vec<Vec<String>>) -> String {
    let mut s = String::new();
    for var in v {
        s.push_str(
            "\n[Event \"?\"]
[Site\"?\"]
[Date \"2022.07.13\"]
[Round \"?\"]
[White \"?\"]
[Black \"?\"]
[Result \"*\"]
[Annotator \"Noor\"]
[PlyCount \"?\"]
[SourceVersionDate \"2022.07.13\"]\n
",
        );
        for pli in var {
            s.push_str(pli);
            s.push(' ');
        }
        s.push_str("*\n");
    }
    s
}
