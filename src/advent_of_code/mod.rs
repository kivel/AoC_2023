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

#[allow(dead_code)]
pub struct RingBuffer<T> {
    buffer: Vec<T>,
    current_index: usize,
}

#[allow(dead_code)]
impl<T: Clone> RingBuffer<T> {
    pub fn new(items: Vec<T>) -> Self {
        RingBuffer {
            buffer: items,
            current_index: 0,
        }
    }

    pub fn next(&mut self) -> T {
        let item = self.buffer[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.buffer.len();
        item
    }
}
