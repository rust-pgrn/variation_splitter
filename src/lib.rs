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
    pub order: Order,
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

        let order = match args.next() {
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
            order,
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
pub struct Data {
    pub variations: Vec<Vec<String>>,
    pub i: usize,
    pub buf: String,
    pub in_comment: bool,
}
impl Default for Data {
    fn default() -> Self {
        let mut variations: Vec<Vec<String>> = Vec::new();
        let i = variations.len();

        variations.push(Vec::new());

        let buf = String::new();

        let in_comment = false;
        Data {
            variations,
            i,
            buf,
            in_comment,
        }
    }
}
impl Data {
    pub fn subtract_i(&mut self, pli: &str) {
        let mut after_brace = true;
        if pli.contains('}') {
            after_brace = false;
        }
        for c in pli.chars() {
            if c == '}' {
                after_brace = true;
            }
            if c == ')' && after_brace {
                self.i -= 1;
            }
        }
    }
    pub fn log(&mut self, pli: &str, len: usize, after_subtract_i: bool, action: String) {
        if !after_subtract_i {
            self.buf.push_str(&format!("Pli: {}\n", pli));
            self.buf.push_str(&format!("Action: {}\n", action));
            self.buf
                .push_str(&format!("Variation: {:?}\n", self.variations[self.i]));
            self.buf
                .push_str(&format!("Working on variation: {}\n", self.i + 1));
            self.buf.push_str(&format!(
                "Number of variations: {}\n",
                self.variations.len()
            ));
            self.buf.push_str(&format!("Pli Number: {}\n", len + 1));
            self.buf
                .push_str(&format!("In Comment: {}\n", self.in_comment));
            self.buf.push_str("\n\n")
        } else {
            self.buf
                .push_str(&format!("Now Working on variation: {}\n", self.i + 1));
            self.buf
                .push_str(&format!("Variation Now: {:?}\n", self.variations[self.i]));
            self.buf.push_str("\n\n");
        }
    }
}

pub fn split(config: &Config) -> Vec<Vec<String>> {
    let contents = contents(config.input_file);

    let contents = clean(contents);
    //-----------------------------------//

    let mut data = Data::default();

    for (len, pli) in contents.split(' ').enumerate() {
        if pli.contains('}') {
            data.in_comment = false;
            if pli.contains(')') {
                data.log(pli, len, false, "Moving Back Variation".to_string());
                data.subtract_i(pli);
                data.log(pli, len, true, "".to_string());
            } else {
                data.log(pli, len, false, "Nothing".to_string());
            }
        } else if pli.contains('{') | data.in_comment {
            if pli.contains('{') {
                data.in_comment = true;
            }
            data.log(pli, len, false, "Nothing".to_string());
        } else if pli.contains(')') {
            let mut action;
            if &pli[0..1] != ")" && !pli.contains('$') {
                data.variations[data.i].push(pli.replace(')', ""));
                action = format!("Adding {} & moving Back Variation", pli);
            } else {
                action = "Moving Back Variation".to_string();
            };
            data.log(pli, len, false, action);
            data.subtract_i(pli);
            data.log(pli, len, true, String::new());
        } else if pli.contains('(') {
            data.variations
                .insert(data.i, data.variations[data.i].clone());
            data.i += 1;
            data.variations[data.i].pop();
            data.log(pli, len, false, "New Variation ".to_string());
        } else if pli.contains('$') | pli.contains("...") {
            data.log(pli, len, false, "Nothing".to_string());
        } else {
            data.variations[data.i].push(pli.to_string());
            data.log(pli, len, false, format!("adding {}", pli));
        }
    }

    if let Order::Side = config.order {
        data.variations.reverse();
        data.buf.push_str("Reversing order");
    };

    fs::write(config.logs_file, data.buf).unwrap();

    data.variations
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
