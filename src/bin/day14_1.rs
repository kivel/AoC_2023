#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day14_1(data: &str) -> usize {
    let d = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dish = advent_of_code::transpose(d);

    // for line in &dish {
    //     println!("{:?}", roll_line(line));
    // }

    dish.iter()
        .map(|line| {
            let line = roll_line(line);
            line.iter()
                .enumerate()
                .filter_map(|(i, &c)| if c == 'O' { Some(i + 1) } else { None })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn roll_line(line: &Vec<char>) -> Vec<char> {
    let mut rolled_line: Vec<char> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();
    for c in line {
        match c {
            'O' => {
                if buffer.is_empty() {
                    rolled_line.push(*c)
                } else {
                    buffer.insert(0, *c);
                    // // rolled_line.push(*c);
                    // rolled_line.extend(&buffer);
                    // buffer = Vec::new();
                }
            }
            '.' => buffer.push(*c),
            '#' => {
                buffer.push(*c);
                rolled_line.extend(&buffer);
                buffer = Vec::new();
            }
            _ => unreachable!(),
        }
    }
    if !buffer.is_empty() {
        rolled_line.extend(&buffer);
    }
    rolled_line.reverse();
    rolled_line
}

fn main() {
    let d = include_str!("../../input/day14.txt");
    let result = day14_1(d);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day14_1, roll_line};

    #[test]
    fn load_calc_test() {
        let input = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        let d = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .rev()
            .collect::<Vec<Vec<char>>>();
        let dish = advent_of_code::transpose(d);
        for line in &dish {
            println!("{:?}", line);
        }
        let s = dish
            .iter()
            .map(|line| {
                line.iter()
                    .enumerate()
                    .filter_map(|(i, &c)| if c == 'O' { Some(i + 1) } else { None })
                    .sum::<usize>()
            })
            .sum::<usize>();
        println!("{s:?}");
        assert_eq!(s, 136)
    }

    #[test]
    fn roll_test() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let d = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            // .rev()
            .collect::<Vec<Vec<char>>>();
        let dish = advent_of_code::transpose(d);
        for line in &dish {
            println!("{:?}", line);
        }
        //single line
        let mut line = dish[0].clone();
        let mut rolled_line = roll_line(&line);

        println!("src:    {line:?}");
        println!("rolled: {rolled_line:?}");
    }

    #[test]
    fn day14_1_test() {
        let d = include_str!("../../input/day14_test.txt");
        let result = day14_1(d);
        println!("{result:?}");
        assert_eq!(result, 136)
    }

    #[test]
    fn day14_1_final() {
        let d = include_str!("../../input/day14.txt");
        let result = day14_1(d);
        println!("{}", result);
        assert_eq!(result, 113456);
    }
}
