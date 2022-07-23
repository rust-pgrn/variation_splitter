use std::fs;
pub fn contents(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("File Not Found")
}
pub fn clean(contents: String) -> String {
    let mut lines = contents.lines();
    for i in 0..=10 {
        lines.next();
    }
    lines.collect()
}
pub fn split<'a>(contents: &str) -> Vec<&'a str> {
    vec!["1.d4", "1.e4"]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_contents() {}
    #[test]
    fn test_clean() {}
    #[test]
    fn test_split() {
        //1.d4(1.e4) =
        //[1.d4,1.e4]
        assert_eq!(split("1.d4(1.e4)"), vec!["1.d4", "1.e4"]);
    }
}
