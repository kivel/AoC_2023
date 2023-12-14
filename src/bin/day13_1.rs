#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day13_1(data: &str) -> usize {
    let iter = data.split("\n\n");

    let maps = iter
        .map(|all_lines| {
            all_lines
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    println!("Received {} maps to check", maps.len());

    maps.iter()
        .map(|this_map| match find_mirror_plane(&this_map) {
            Some(mirror) => mirror * 100usize,
            None => {
                println!("no mirror plane found in rows");
                let map_prime = advent_of_code::transpose(this_map.clone());
                find_mirror_plane(&map_prime).unwrap_or(0)
            }
        })
        .sum::<usize>()
}

fn find_mirror_plane(input: &Vec<Vec<char>>) -> Option<usize> {
    let idx = input
        .windows(2)
        .enumerate()
        .filter_map(|(i, w)| if w[0] == w[1] { Some(i) } else { None })
        .collect::<Vec<usize>>();
    println!("idx: {:?}", idx);

    // list if mirror plane indexes, the plane is actually a pair at index and index + 1
    if idx.len() == 0 {
        return None;
    }

    // use a widen a windows to index-1 and index+2 and check if they are the same
    // if either index hits the end of the vector, the plane is valid, return the index+1
    // do this for EACH pair of indexes!!!

    let p = idx
        .iter()
        .map(|i| {
            let mut left = *i;
            let mut right = left + 1;
            let mut valid = true;
            println!("checking {i}");
            while left > 0 && right < input.len() - 1 {
                left -= 1;
                right += 1;
                println!("left: {left}, right: {right}");
                if input[left] != input[right] {
                    println!("{:?}", input[left]);
                    println!("{:?}", input[right]);
                    valid = false;
                    break;
                }
            }
            match valid && (left == 0 || right == input.len() - 1) {
                true => i + 1,
                false => 0,
            }
        })
        .sum();
    println!("=====> p: {:?}", p);
    if p == 0 {
        return None;
    }
    Some(p)
}
fn main() {
    let d = include_str!("../../input/day13.txt");
    let result = day13_1(d);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::day13_1;

    #[test]
    fn day13_1_test() {
        let d = include_str!("../../input/day13_test.txt");
        let result = day13_1(d);
        assert_eq!(result, 405);
    }

    #[test]
    fn day13_1_final() {
        let d = include_str!("../../input/day13.txt");
        let result = day13_1(d);
        println!("{}", result);
        assert_eq!(result, 35691);
    }
}
