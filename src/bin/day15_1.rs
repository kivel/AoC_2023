#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day14_1(data: &str) -> usize {
    data.split(',').map(|s| hasher(s)).sum::<usize>()
}

fn main() {
    let d = include_str!("../../input/day15.txt");
    let result = day14_1(d);
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
#[cfg(test)]
mod tests {
    use crate::{day14_1, hasher};

    #[test]
    fn test_hasher() {
        let input = "HASH";
        let hash = hasher(input);
        assert_eq!(hash, 52)
    }
    #[test]
    fn day14_1_test() {
        let d = include_str!("../../input/day15_test.txt");
        let result = day14_1(d);
        println!("{result:?}");
        assert_eq!(result, 1320)
    }

    #[test]
    fn day14_1_final() {
        let d = include_str!("../../input/day15.txt");
        let result = day14_1(d);
        println!("{}", result);
        assert_eq!(result, 506891);
    }
}
