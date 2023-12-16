use std::collections::HashMap;
use std::fmt::Debug;
use std::num::ParseIntError;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

//rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7

// rn=1
// label = rn
// f=1
#[derive(Debug, Clone, Eq, PartialEq)]
struct Lens {
    label: String,
    f: Result<usize, ParseIntError>,
}

impl Lens {
    fn from_string(s: &str) -> Self {
        let split: Vec<&str> = s.split(&['=', '-']).collect();
        let label = split[0];
        let f = split[1].parse::<usize>();
        Self {
            label: label.to_owned(),
            f,
        }
    }

    fn show(&self) {
        println! {"label: {}, f={:?}", self.label, self.f};
    }
}

fn day15_2(data: &str) -> usize {
    let mut Boxes: HashMap<usize, Vec<Lens>> = HashMap::new();
    // All 256 boxes are always present;
    for i in 0..256 {
        Boxes.insert(i, vec![]);
    }
    let lenses = data
        .split(',')
        .map(|s| Lens::from_string(s))
        .collect::<Vec<Lens>>();

    lenses.iter().for_each(|lens| {
        let key = hasher(lens.label.as_str());
        Boxes.entry(key).and_modify(|x| {
            if lens.f.is_ok() {
                match find_lens_in_box(x, lens.label.as_str()) {
                    None => x.push(lens.clone()),
                    Some(i) => x[i] = lens.clone(),
                }
            } else {
                x.retain(|l| l.label != lens.label);
            }
        });
    });

    Boxes
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(b, l)| {
            l.iter()
                .enumerate()
                .map(|(i, l)| (b + 1) * (i + 1) * l.clone().f.unwrap())
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    let d = include_str!("../../input/day15.txt");
    let result = day15_2(d);
    println!("{}", result);
}

fn hasher(input: &str) -> usize {
    let mut hash = 0;
    for b in input.as_bytes() {
        hash += *b as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn find_lens_in_box(lenses: &Vec<Lens>, label: &str) -> Option<usize> {
    lenses
        .iter()
        .enumerate()
        .filter_map(|(i, l)| if l.label == label { Some(i) } else { None })
        .nth(0)
}
#[cfg(test)]
mod tests {
    use crate::{day15_2, hasher, Lens};
    use regex::Regex;

    #[test]
    fn sandbox() {
        let input = "rn=3";
        let split: Vec<&str> = input.split(&['=', '-']).collect();
        let label = split[0];
        let f = split[1].parse::<usize>();
        println!("{:?}", label);
        println!("{:?}", f);
        // let re = Regex::new(r"(\w*)?(=|-)(\d)?").unwrap();
        // re.split()
        // for mat in re.find_iter(input) {
        //     println!("{:?}", mat);
        // }
        println! {"input: {} -> label: {} -> box: {}", input, label, hasher(label)};
        let lens = Lens::from_string(input);
        println! {"label: {}, f={:?}", lens.label, lens.f};
        lens.show();
    }
    #[test]
    fn test_hasher() {
        let input = "HASH";
        let hash = hasher(input);
        assert_eq!(hash, 52)
    }
    #[test]
    fn day15_2_test() {
        let d = include_str!("../../input/day15_test.txt");
        let result = day15_2(d);
        println!("{result:?}");
        assert_eq!(result, 145)
    }

    #[test]
    fn day15_2_final() {
        let d = include_str!("../../input/day15.txt");
        let result = day15_2(d);
        println!("{}", result);
        assert_eq!(result, 506891);
    }
}
