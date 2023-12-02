#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day1_1(data: &Vec<String>) -> u32 {
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
fn main() {

    let d = advent_of_code::Reader::read_file("./input/day1_1_test.txt").unwrap();
    let sum = day1_1(&d);
    println!("result: {sum}");

}