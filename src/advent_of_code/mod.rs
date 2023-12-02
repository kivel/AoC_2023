use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub mod aoc {}
pub struct Reader {}

impl Reader {
    // Returns an Iterator to the Reader of the lines of the file.
    pub fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(std::io::BufReader::new(file).lines())
    }

    pub fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
        let org_data = match Reader::read_lines(filename) {
            Ok(lines) => lines.collect::<Result<_, _>>().unwrap(),
            Err(e) => panic!("Error opening file {}: {}", filename, e),
        };
        Ok(org_data)
    }
}
