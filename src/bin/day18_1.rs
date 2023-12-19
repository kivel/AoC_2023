use std::collections::VecDeque;
use std::num::ParseIntError;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;
fn day18_1(data: &str) -> usize {
    let mut points: Vec<Point> = vec![Point { x: 0, y: 0 }];

    data.lines()
        .map(|l| {
            let v = l.split_whitespace().collect::<Vec<&str>>();
            (v[0], v[1].parse::<i32>())
        })
        .for_each(|instr| {
            let np = next_point(&points.iter().last().unwrap(), instr);
            points.push(np.unwrap());
        });

    let mut max_x = 0;
    let mut min_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;
    points.iter().for_each(|point| {
        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
        min_x = min_x.min(point.x);
        min_y = min_y.min(point.y);
    });
    let n_x = max_x - min_x;
    let n_y = max_y - min_y;
    println!("max_x = {}, max_y = {}", max_x, max_y);
    println!("min_x = {}, min_y = {}", min_x, min_y);
    println!("n_x = {}, n_y = {}", n_x, n_y);

    // start with empty space, aka '.'
    let mut field = vec![vec!['.'; 1 + n_x as usize]; 1 + n_y as usize];
    // dig drenches by placing '#' on the perimeter
    points.windows(2).for_each(|pair| {
        for x in (pair[0].x.min(pair[1].x)..=pair[0].x.max(pair[1].x)).into_iter() {
            for y in (pair[0].y.min(pair[1].y)..=pair[0].y.max(pair[1].y)).into_iter() {
                // println!("setting [{}][{}]", &y, &x);
                field[(y - min_y) as usize][(x - min_x) as usize] = '#';
            }
        }
    });

    let mut total = 0;
    field.iter().enumerate().for_each(|(i, p)| {
        let mut new_line: Vec<char> = Vec::new();
        let mut cnt = 0;
        let mut dq: VecDeque<char> = VecDeque::from(p.clone());
        // print!("{i}: ");
        // let mut last = 'x';
        let mut inside = false;
        while let Some(c) = dq.pop_front() {
            // print!("{c}");
            match c {
                '#' => {
                    inside = true;
                    cnt += 1;
                    new_line.push(c);
                }
                '.' => {
                    if inside {
                        if !dq.contains(&'#') {
                            inside = false;
                            continue;
                        }
                        cnt += 1;
                        new_line.push('#');
                        continue;
                    }
                    new_line.push(c);
                }
                _ => unreachable!(),
            }
        }
        total += cnt;
        // println!("{new_line:?}");
    });

    // println!("AREA = {total}");

    // for p in &field {
    //     for c in p {
    //         print!("{c}");
    //     }
    //     println!();
    // }

    // field.windows(2).for_each(|pair| {
    //     let p = (*pair[0],*pair[1]);
    //     match p {
    //         ('#','.') => { println!("")}
    //     }
    // });
    // let words = field
    //     .iter()
    //     .map(|vc| vc.iter().collect::<String>())
    //     .collect::<Vec<String>>();
    // println!("{:?}", words);

    // let mut x: usize = 0;
    // let mut y: usize = 0;
    // let np = next_point(&points.iter().last().unwrap(), ("R", Ok(6)));
    // println!("{:?}", np);
    // for p in &points {
    //     println!("{:?}", p);
    // }
    // let a = calculate_area(&points);
    // println!("{:?}", a);
    // let p = calculate_perimeter(&points);
    // println!("{:?}", p);
    total
}

fn next_point(current: &Point, instruction: (&str, Result<i32, ParseIntError>)) -> Option<Point> {
    let d = instruction.1.unwrap();
    match instruction.0 {
        "R" => Some(Point {
            x: current.x + d,
            y: current.y,
        }),
        "L" => Some(Point {
            x: current.x - d,
            y: current.y,
        }),
        "U" => Some(Point {
            x: current.x,
            y: current.y - d,
        }),
        "D" => Some(Point {
            x: current.x,
            y: current.y + d,
        }),
        _ => unreachable!(),
    }
}

fn main() {
    let d = include_str!("../../input/day18_test.txt");
    let result = day18_1(d);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::day18_1;

    #[test]
    fn day18_1_test() {
        let d = include_str!("../../input/day18_test.txt");
        let result = day18_1(d);
        println!("{result:?}");
        assert_eq!(result, 62)
    }

    #[test]
    fn day18_1_final() {
        let d = include_str!("../../input/day18.txt");
        let result = day18_1(d);
        println!("{}", result);
        assert!(result < 83749);
    }
}
