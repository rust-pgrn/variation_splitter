use std::{fs, ops::Deref};
pub fn contents(filename: &str) -> String {
    fs::read_to_string(filename).expect("File Not Found")
}
pub fn clean(contents: String) -> String {
    let mut lines = contents.lines();
    for _ in 0..=10 {
        lines.next();
    }
    lines.collect()
}
pub fn split(contents: String) -> Vec<Vec<String>> {
    let mut variations: Vec<Vec<String>> = Vec::new();
    let mut i = variations.len();
    let mut j = 1;
    variations.push(Vec::new());
    for pli in contents.split(' ') {
        //println!("{}", pli);
        if pli.contains("...") && pli.contains("(") {
            //Creates new array with current moves
            variations.push(variations[i].clone());
            //increments i to work on next array/variation
            i += j;
            //Removes last 3 moves, which will get replaced by the current pli
            variations[i].pop();
            variations[i].pop();
            variations[i].pop();
        } else if pli.contains('(') {
            //Creates new array with current moves
            variations.push(variations[i].clone());
            //increments i to work on next array/variation
            i += j;
            //Removes last 2 moves, which will get replaced by the current pli
            variations[i].pop();
            variations[i].pop();
            variations[i].push(pli.replace('(', ""));
        } else if pli.contains(')') {
            variations[i].push(pli.replace(')', ""));
            i -= 1;
            j += 1;
        } else if pli.contains("...") && !pli.contains('(') {
        } else {
            variations[i].push(pli.to_string());
        }
        //variations.last_mut().unwrap().push(pli);
    }
    variations
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
