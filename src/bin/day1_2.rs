#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;
fn day1_2(data: &Vec<String>) -> u32 {
    let data = replace_number_literals_s(data);
    data.iter()
        .map(|l| {
            let characters = l.chars();
            let digits = &characters
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect::<Vec<u32>>();
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
}

fn replace_number_literals_s(data: &Vec<String>) -> Vec<String> {
    data.iter()
        .map(|l| {
            let mut l = l.to_string();
            l = l.replace("one", "o1e");
            l = l.replace("two", "t2o");
            l = l.replace("three", "t3e");
            l = l.replace("four", "4");
            l = l.replace("five", "5e");
            l = l.replace("six", "6");
            l = l.replace("seven", "7n");
            l = l.replace("eight", "e8t");
            l = l.replace("nine", "n9e");
            l
        })
        .collect::<Vec<String>>()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day1_2.txt").unwrap();
    let sum = day1_2(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day1_2};

    #[test]
    fn day2_ind() {
        let result = day1_2(&vec!["two1nine".to_string()]);
        assert_eq!(result, 29);
        let result = day1_2(&vec!["eightwothree".to_string()]);
        assert_eq!(result, 83);
        let result = day1_2(&vec!["abcone2threexyz".to_string()]);
        assert_eq!(result, 13);
        let result = day1_2(&vec!["xtwone3four".to_string()]);
        assert_eq!(result, 24);
        let result = day1_2(&vec!["4nineeightseven2".to_string()]);
        assert_eq!(result, 42);
        let result = day1_2(&vec!["zoneight234".to_string()]);
        assert_eq!(result, 14);
        let result = day1_2(&vec!["7pqrstsixteen".to_string()]);
        assert_eq!(result, 76);
    }
    #[test]
    fn day2_res() {
        let d = advent_of_code::Reader::read_file("./input/day1_2_test.txt").unwrap();
        let result = day1_2(&d);
        assert_eq!(result, 281);
    }

    #[test]
    fn day2_final() {
        let d = advent_of_code::Reader::read_file("./input/day1_2.txt").unwrap();
        let result = day1_2(&d);
        assert_eq!(result, 53539);
    }
}
