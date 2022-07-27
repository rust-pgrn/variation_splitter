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
pub fn split(contents: String) -> Vec<Vec<String>> {
    let mut variations: Vec<Vec<String>> = Vec::new();
    let mut route = Vec::new();
    let mut i = variations.len();
    let mut len = 0;
    route.push(i);
    variations.push(Vec::new());
    for pli in contents.split(' ') {
        if pli.contains('(') {
            //Creates new array with current moves
            variations.push(variations[i].clone());
            route.push(variations.len() - 1);
            i = *route.last().unwrap();
            variations[i].pop();
        } else if pli.contains(')') {
            variations[i].push(pli.replace(')', ""));
            route.pop();
            i = *route.last().unwrap();
        } else if pli.contains("...") {
        } else {
            variations[i].push(pli.to_string());
        }
        if len < 50 {
            println!("Pli: {}", pli);
            println!("Variation: {:?}", variations[i]);
            println!("Working on variation: {}", i);
            println!("Number of variations: {}", variations.len());
            print!("\n\n");
            len += 1;
        }
    }
    //println!("{:?}", variations);
    println!("{}", variations.len());
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
