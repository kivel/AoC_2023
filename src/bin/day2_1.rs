#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug)]
struct Record {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
}

impl Record {
    pub fn from_string(s: &str) -> Record {
        let mut result = Record {
            red: 0,
            green: 0,
            blue: 0,
        };

        let mut cubes = s.split(',').collect::<Vec<&str>>();
        while !cubes.is_empty() {
            let cube = cubes
                .pop()
                .unwrap()
                .trim()
                .split(' ')
                .collect::<Vec<&str>>();

            let mut it = cube.iter();
            let color = it.next().unwrap().parse::<u32>().unwrap();
            match it.last() {
                Some(&"red") => result.red = color,
                Some(&"green") => result.green = color,
                Some(&"blue") => result.blue = color,
                _ => {}
            }
        }
        result
    }
}
fn main() {
    let bag = Record {
        red: 12,
        green: 13,
        blue: 14,
    };

    let d = advent_of_code::Reader::read_file("./input/day2_1.txt").unwrap();

    let mut sum = 0;

    for line in d {
        let l = line.split(':').collect::<Vec<&str>>();
        let game = l
            .iter()
            .next()
            .unwrap()
            .split(' ')
            .last()
            .expect("number")
            .parse::<u32>()
            .unwrap();
        let draws = l
            .last()
            .unwrap()
            .trim()
            .split(';')
            .map(|g| Record::from_string(g))
            .collect::<Vec<Record>>();
        if draws.iter().all(|d| *d <= bag) {
            sum += game;
        }
    }
    println!("result: {sum}");
}
