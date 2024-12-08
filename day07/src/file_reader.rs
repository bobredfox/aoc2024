pub mod file_handling {
use std::fs;

   pub fn read_file(input: &str) -> Option<String> {
    match fs::read_to_string(input) {
        Ok(x) => Some(x.to_string()),
        Err(..) => None,
    }
   }

   pub fn give_lines_of_file(path: &str) -> Option<Vec<String>> {
    match read_file(path) {
        Some(s) => Some(s.lines().map(|x| x.to_string()).collect()),
        None => None,
   }
   }
}    
