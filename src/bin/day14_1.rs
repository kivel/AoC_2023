#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day14_1(data: &str) -> usize {
    todo!()
}

fn main() {
    let d = include_str!("../../input/day14.txt");
    let result = day14_1(d);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day14_1};

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
    fn day14_1_test() {
        let d = include_str!("../../input/day14_test.txt");
        let result = day14_1(d);
        assert_eq!(result, 405);
    }

    #[test]
    fn day14_1_final() {
        let d = include_str!("../../input/day14.txt");
        let result = day14_1(d);
        println!("{}", result);
        assert_eq!(result, 35691);
    }
}
