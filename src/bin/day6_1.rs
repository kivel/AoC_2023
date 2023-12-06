use std::{isize, usize};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

// d = t_move * v
// d = t - t_charge * v
// v = t_charge
// d = (t - t_charge) * t_charge
// d = t*t_charge - t_charge^2
// 0 = t*t_charge - t_charge^2 - d
fn day6_1(data: &Vec<(isize, isize)>) -> usize {
    data.iter()
        .map(|(t, d)| find_t_charge(t, d))
        .map(|roots| (roots.0..=roots.1).collect::<Vec<isize>>().len())
        .collect::<Vec<usize>>()
        .iter()
        .inspect(|p| println!("{p} possibilities"))
        .product()
    // 0
}

fn find_t_charge(t: &isize, d: &isize) -> (isize, isize) {
    // tc1 = 1/2 (t - sqrt(t^2 - 4 d))
    // tc2 = 1/2 (sqrt(t^2 - 4 d) + t)

    let discriminant = t * t - 4 * d;

    if discriminant < 0 {
        // No real roots, meaning no valid t_charge values
        panic!()
    }

    let sqrt_discriminant = (discriminant as f64).sqrt();

    let mut tc1 = (*t as f64 - sqrt_discriminant) / 2.0;
    let mut tc2 = (sqrt_discriminant + *t as f64) / 2.0;
    if tc1.fract() == 0.0 {
        tc1 += 1.0;
    }
    if tc2.fract() == 0.0 {
        tc2 -= 1.0;
    }
    let tc1 = tc1.ceil();
    let tc2 = tc2.floor();
    println!("for t={t} ms and d={d} mm, tc1={tc1} and tc2={tc2}");
    (tc1 as isize, tc2 as isize)
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day6_1.txt").unwrap();

    // let sum = day6_1(&d);
    // println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day6_1, find_t_charge};

    #[test]
    fn test_sandbox() {
        let roots = find_t_charge(&7, &9);
        let v = (roots.0..=roots.1).collect::<Vec<isize>>();
        println!("roots: {:?} -> {:?} with len {}", roots, v, v.len());
    }
    #[test]
    fn test_res() {
        // Time:      7  15   30
        // Distance:  9  40  200
        let d = vec![(7isize, 9isize), (15isize, 40isize), (30isize, 200isize)];
        // let d = advent_of_code::Reader::read_file("./input/day6_1_test.txt").unwrap();
        let result = day6_1(&d);
        println!("result: {result}");
        assert_eq!(result, 288);
    }

    #[test]
    fn test_final() {
        let d = advent_of_code::Reader::read_file("./input/day6_1.txt").unwrap();
        // let result = day6_1(&d);
        // assert_eq!(result, 484023871);
    }
}
