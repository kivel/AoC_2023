#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day12_1(data: &Vec<String>) -> usize {
    data.iter()
        .map(|input| {
            let parts: Vec<&str> = input.split_whitespace().collect();
            let pattern = parts[0];

            let validations: Vec<usize> = parts[1]
                .trim_end_matches(',')
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            let mut all_patterns: Vec<Vec<char>> = Vec::new();
            let mut current: Vec<char> = pattern.chars().collect();

            generate_combinations(pattern, &mut current, 0, &validations, &mut all_patterns);
            all_patterns.len()
        })
        // .inspect(|l| println!("{:?}", l))
        .sum::<usize>()
}

// inspired by chatGPT
fn generate_combinations(
    pattern: &str,
    current: &mut Vec<char>,
    index: usize,
    validations: &Vec<usize>,
    all_patterns: &mut Vec<Vec<char>>,
) {
    if index == pattern.len() {
        // collect Vec<char> into String, split by '.' and map the length to the '#' groups if any.
        let groups = current
            .iter()
            .collect::<String>()
            .split('.')
            .filter_map(|s| if s.contains('#') { Some(s.len()) } else { None })
            .collect::<Vec<usize>>();

        if groups == *validations {
            all_patterns.push(current.clone());
        }

        return;
    }

    match pattern.chars().nth(index) {
        Some(ch) if ch == '.' || ch == '#' => {
            current[index] = ch;
            generate_combinations(pattern, current, index + 1, validations, all_patterns);
        }
        Some(ch) if ch == '?' => {
            // Replace "?" with "." and recurse
            current[index] = '.';
            generate_combinations(pattern, current, index + 1, validations, all_patterns);

            // Replace "?" with "#" and recurse
            current[index] = '#';
            generate_combinations(pattern, current, index + 1, validations, all_patterns);
        }
        _ => unreachable!(),
    }
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day12_1.txt").unwrap();

    let result = day12_1(&d);
    println!("{result:?}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day12_1};

    #[test]
    fn get_groups() {
        let data = ".#...#....###.";
        let groups = data
            .split('.')
            .filter_map(|s| if s.contains('#') { Some(s.len()) } else { None })
            .collect::<Vec<usize>>();
        assert_eq!(groups, vec![1, 1, 3])
    }
    #[test]
    fn test_validator() {
        // let data = "#.#.### 1,1,3".to_owned();
        let data = ".#...#....###. 1,1,3".to_owned();

        let split = data.split_whitespace().collect::<Vec<&str>>();
        let rec_gro = split[0]
            .split('.')
            .filter_map(|s| if s.contains('#') { Some(s.len()) } else { None })
            .collect::<Vec<usize>>();
        let groups: Vec<usize> = split[1].split(',').filter_map(|s| s.parse().ok()).collect();
        println!("{rec_gro:?}, {groups:?}");
        println!("{}", rec_gro == groups);
    }
    #[test]
    fn day12_test() {
        let d = advent_of_code::Reader::read_file("./input/day12_1_test.txt").unwrap();
        let result = day12_1(&d);
        assert_eq!(result, 21);
    }

    #[test]
    fn day12_final() {
        let d = advent_of_code::Reader::read_file("./input/day12_1.txt").unwrap();
        let result = day12_1(&d);
        assert_eq!(result, 7047);
    }
}
