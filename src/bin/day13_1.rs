use std::collections::HashMap;

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
    let mut line_map: HashMap<Vec<char>, Vec<usize>> = HashMap::new();

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

    // while left >= 0 && right < input.len() {
    //     left -= 1;
    //     right += 1;
    //     if input[left] == input[right] {
    //         continue;
    //     }
    // }

    // input.iter().enumerate().for_each(|(i, line)| {
    //     line_map
    //         .entry(line.clone())
    //         .and_modify(|counter| counter.push(i))
    //         .or_insert(vec![i]);
    // });
    //
    // // fails here if there are three identical lines, but ont is out of bounds
    // let mut mirrored_lines = line_map
    //     .iter()
    //     .filter(|(_, v)| v.len() >= 2)
    //     .flat_map(|(_, v)| v.clone())
    //     .collect::<Vec<usize>>();
    //
    // // bail if no mirrored lines at all
    // if mirrored_lines.len() == 0 {
    //     return None;
    // }
    //
    // mirrored_lines.sort();
    // println!("mirrored lines: {mirrored_lines:?}");
    // println!("{:?}", line_map);
    //
    // // bail if not all mirrored lines are consecutive
    // if !mirrored_lines.windows(2).all(|w| w[0] == w[1] - 1) {
    //     return None;
    // }
    //
    // match mirrored_lines[0] == 0 || *mirrored_lines.iter().last().unwrap() == input.len() - 1 {
    //     true => {
    //         let lines_to_mirror = mirrored_lines[mirrored_lines.len() / 2];
    //         println!("perfect mirror at {lines_to_mirror}");
    //         println!("mirrored lines: {mirrored_lines:?}");
    //         return Some(lines_to_mirror);
    //     }
    //     false => {
    //         return None;
    //     }
    // }
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
