use std::{isize, usize};

// d = t_move * v
// d = t - t_charge * v
// v = t_charge
// d = (t - t_charge) * t_charge
// d = t*t_charge - t_charge^2
// 0 = t*t_charge - t_charge^2 - d
fn day6_1(data: &Vec<(isize, isize)>) -> usize {
    // mathematical solution
    data.iter()
        .map(|(t, d)| find_t_charge(t, d))
        .map(|roots| (roots.0..=roots.1).collect::<Vec<isize>>().len())
        .collect::<Vec<usize>>()
        .iter()
        .product()
}

fn calc_distance(t: &isize, d: &isize) -> usize {
    // let valid_charges: Vec<isize> = Vec::new();
    let values = (0..*t)
        .inspect(|d| println!("t_c = {d}"))
        .filter_map(|t_charge| {
            let dist = (t - &t_charge) * &t_charge;
            if &dist > d {
                Some(dist)
            } else {
                None
            }
        })
        .inspect(|d| println!("d= {d}"))
        .collect::<Vec<isize>>()
        .len();
    values
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
    // deal with integer result. neither floor nor ceil catches this.
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
    // Time:        44     70     70     80
    // Distance:   283   1134   1134   1491
    let d: Vec<(isize, isize)> = vec![(44, 283), (70, 1134), (70, 1134), (80, 1491)];
    let result = day6_1(&d);
    println!("result: {result}");
}

#[cfg(test)]
mod tests {
    use crate::{calc_distance, day6_1, find_t_charge};

    #[test]
    fn test_res() {
        // Time:      7  15   30
        // Distance:  9  40  200
        let d: Vec<(isize, isize)> = vec![(7, 9), (15, 40), (30, 200)];
        let result = day6_1(&d);
        println!("result: {result}");
        assert_eq!(result, 288);
    }

    #[test]
    fn iteration() {
        // iterates over the data, terrible time complexity
        let input: Vec<(isize, isize)> = vec![(44, 283), (70, 1134), (70, 1134), (80, 1491)];
        let result: usize = input
            .iter()
            .map(|(t, d)| calc_distance(t, d))
            .collect::<Vec<usize>>()
            .iter()
            .product();
        assert_eq!(result, 219849);
    }

    #[test]
    fn mathematical() {
        // iterates over the data, terrible time complexity
        let input: Vec<(isize, isize)> = vec![(44, 283), (70, 1134), (70, 1134), (80, 1491)];
        let result: usize = input
            .iter()
            .map(|(t, d)| find_t_charge(t, d))
            .map(|roots| (roots.0..=roots.1).collect::<Vec<isize>>().len())
            .collect::<Vec<usize>>()
            .iter()
            .product();
        assert_eq!(result, 219849);
    }

    #[test]
    fn test_final() {
        let input: Vec<(isize, isize)> = vec![(44, 283), (70, 1134), (70, 1134), (80, 1491)];
        let result = day6_1(&input);
        assert_eq!(result, 219849);
    }
}
