use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::Range;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone)]
struct Map {
    lookup: HashMap<Range<usize>, Range<usize>>,
}

impl Map {
    fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    fn push(&mut self, map: Vec<usize>) {
        let dst_start = map[0];
        let src_start = map[1];
        let range = map[2];
        let dst = dst_start..dst_start + range;
        let src = src_start..src_start + range;
        self.lookup.insert(src, dst);
    }

    fn get(&self, key: usize) -> usize {
        let starts = self
            .lookup
            .iter()
            .filter_map(|(src, dst)| {
                if src.contains(&key) {
                    Some((dst.start, src.start))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, usize)>>();

        if let Some((dst_start, src_start)) = starts.first() {
            key - *src_start + *dst_start
        } else {
            key
        }
    }
}

fn day5_2(data: &Vec<String>) -> usize {
    let mut seeds: Vec<usize> = Vec::new();
    let mut seed_to_soil_map: Map = Map::new();
    let mut soil_to_fertilizer_map: Map = Map::new();
    let mut fertilizer_to_water_map: Map = Map::new();
    let mut water_to_light_map: Map = Map::new();
    let mut light_to_temperature_map: Map = Map::new();
    let mut temperature_to_humidity_map: Map = Map::new();
    let mut humidity_to_location_map: Map = Map::new();

    let mut current_map: Option<&mut Map> = None;

    // this is heavily inspired by chatGPT-3.5
    let mut input = data.clone();
    input.reverse();
    while !input.is_empty() {
        let line = input.pop().unwrap();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("seeds:") {
            let seed_pairs = line
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();
            seed_pairs
                .iter()
                .enumerate()
                .step_by(2)
                .for_each(|(i, start)| {
                    let end = start + seed_pairs[i + 1];
                    seeds.extend((*start..end).collect::<Vec<usize>>());
                });
            continue; // seeds is in a single line
        }
        println!("building map from: {}", line);
        if line == "seed-to-soil map:" {
            current_map = Some(&mut seed_to_soil_map);
            continue;
        }
        if line == "soil-to-fertilizer map:" {
            current_map = Some(&mut soil_to_fertilizer_map);
            continue;
        }
        if line == "fertilizer-to-water map:" {
            current_map = Some(&mut fertilizer_to_water_map);
            continue;
        }
        if line == "water-to-light map:" {
            current_map = Some(&mut water_to_light_map);
            continue;
        }
        if line == "light-to-temperature map:" {
            current_map = Some(&mut light_to_temperature_map);
            continue;
        }
        if line == "temperature-to-humidity map:" {
            current_map = Some(&mut temperature_to_humidity_map);
            continue;
        }
        if line == "humidity-to-location map:" {
            current_map = Some(&mut humidity_to_location_map);
            continue;
        }
        let Some(ref mut map) = current_map else {
            panic!("no map found");
        };
        {
            let numbers: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            map.push(numbers);
        }
    }
    // println!("seed_to_soil_map: {:?}", seed_to_soil_map);
    // println!("soil_to_fertilizer_map: {:?}", soil_to_fertilizer_map);
    // println!("fertilizer_to_water_map: {:?}", fertilizer_to_water_map);
    // println!("water_to_light_map: {:?}", water_to_light_map);
    // println!("light_to_temperature_map: {:?}", light_to_temperature_map);
    // println!(
    //     "temperature_to_humidity_map: {:?}",
    //     temperature_to_humidity_map
    // );
    // println!("humidity_to_location_map: {:?}", humidity_to_location_map);
    println!("seeds: {:?}", seeds.len());

    seeds
        .par_iter()
        .map(|seed| seed_to_soil_map.get(*seed))
        .map(|soil| soil_to_fertilizer_map.get(soil))
        .map(|fertilizer| fertilizer_to_water_map.get(fertilizer))
        .map(|water| water_to_light_map.get(water))
        .map(|humidity| light_to_temperature_map.get(humidity))
        .map(|temp| temperature_to_humidity_map.get(temp))
        .map(|humidity| humidity_to_location_map.get(humidity))
        .min()
        .unwrap()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day5_1.txt").unwrap();

    let sum = day5_2(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day5_2};

    #[test]
    fn sandbox() {
        let start = 0;
        let end: usize = 10;
        let v = (start..end).collect::<Vec<usize>>();
        println!("{:?}", v);
    }
    #[test]
    fn day5_res() {
        let d = advent_of_code::Reader::read_file("./input/day5_1_test.txt").unwrap();
        let result = day5_2(&d);
        println!("result: {result}");
        assert_eq!(result, 46);
    }

    #[test]
    fn day5_final() {
        let d = advent_of_code::Reader::read_file("./input/day5_1.txt").unwrap();
        let result = day5_2(&d);
        assert_eq!(result, 484023871);
    }
}
