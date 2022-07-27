use std::fs;
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
//ouoeuoeuoeu
pub fn split(contents: String, filename: &str) -> Vec<Vec<String>> {
    let mut variations: Vec<Vec<String>> = Vec::new();
    let mut i = variations.len();
    variations.push(Vec::new());
    let mut buf = String::new();
    for (len, pli) in contents.split(' ').enumerate() {
        if pli.contains('(') {
            //Creates new array with current moves
            variations.push(variations[i].clone());
            variations[i].pop();
            //if len < 470 && len > 380 {
            buf.push_str(&format!("Pli: {}\n", pli));
            buf.push_str(&format!("Variation: {:?}\n", variations[i]));
            buf.push_str(&format!("Working on variation: {}\n", i + 1));
            buf.push_str(&format!("Number of variations: {}\n", variations.len()));
            buf.push_str(&format!("Pli Number: {}\n", len + 1));
            buf.push_str("\n\n");
            println!("{}", &buf);
            //}
        } else if pli.contains(')') {
            variations[i].push(pli.replace(')', ""));
            //if len < 470 && len > 380 {
            buf.push_str(&format!("Pli: {}\n", pli));
            buf.push_str(&format!("Variation: {:?}\n", variations[i]));
            buf.push_str(&format!("Working on variation: {}\n", i + 1));
            buf.push_str(&format!("Number of variations: {}\n", variations.len()));
            buf.push_str(&format!("Pli Number: {}\n", len + 1));
            //}
            buf.push_str(&format!("Now Working on variation: {}\n", i + 1));
            buf.push_str("\n\n");
            println!("{}", &buf);
        } else if pli.contains("...") | pli.contains('$') {
            //if len < 470 && len > 380 {
            buf.push_str(&format!("Pli: {}\n", pli));
            buf.push_str(&format!("Variation: {:?}\n", variations[i]));
            buf.push_str(&format!("Working on variation: {}\n", i + 1));
            buf.push_str(&format!("Number of variations: {}\n", variations.len()));
            buf.push_str(&format!("Pli Number: {}\n", len + 1));
            buf.push_str("\n\n");
            println!("{}", &buf);
            //}
        } else {
            variations[i].push(pli.to_string());
            //if len < 470 && len >380 {
            buf.push_str(&format!("Pli: {}\n", pli));
            buf.push_str(&format!("Variation: {:?}\n", variations[i]));
            buf.push_str(&format!("Working on variation: {}\n", i + 1));
            buf.push_str(&format!("Number of variations: {}\n", variations.len()));
            buf.push_str(&format!("Pli Number: {}\n", len + 1));
            buf.push_str("\n\n");
            println!("{}", &buf);
            //}
        }
    }
    //println!("{:?}", variations);
    //println!("{}", variations.len());
    let _s = convert(&variations);
    //println!("{}", buf);
    fs::write(filename, buf).unwrap();
    variations
}
pub fn convert(v: &Vec<Vec<String>>) -> String {
    let mut s = String::new();
    for var in v {
        for pli in var {
            s.push_str(pli);
            s.push(' ');
        }
        s.push_str("\n\n\n\n\n\n\n");
    }
    s
}
#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn test_contents() {}
    #[test]
    fn test_clean() {}
    #[test]
    fn test_split() {
        //1.d4(1.e4) =
        //[1.d4,1.e4]
        //assert_eq!(split("1.d4(1.e4)"), vec!["1.d4", "1.e4"]);
    }
}
