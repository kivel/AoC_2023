#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

struct History {
    data: Vec<i32>,
}

impl History {
    fn extrapolate(&self) -> i32 {
        // decay series
        let mut series: Vec<Vec<i32>> = Vec::new();
        series.push(self.data.clone());
        // working copy
        let mut work_series: Vec<i32> = self.data.clone();

        loop {
            // clean vector
            let mut tmp: Vec<i32> = Vec::new();
            // differences
            for i in 0..work_series.len() - 1 {
                tmp.push(work_series[i + 1] - work_series[i]);
            }
            series.push(tmp.clone()); // add to decay series
            work_series = tmp.clone(); // replace working copy for next iteration
                                       // fully decayed?
            if tmp.iter().all(|x| *x == 0) {
                let mut e = 0;
                let v = series.iter().rev().map(|x| x[0]).collect::<Vec<i32>>();
                for i in 1..v.len() {
                    e = v[i] - e;
                }
                return e;
            }
        }
    }
}

fn day9_1(data: &Vec<String>) -> i32 {
    data.iter()
        .map(|input| {
            input
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .map(|v| {
            let history = History { data: v };
            history.extrapolate()
        })
        .sum::<i32>()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day9_1.txt").unwrap();
    let result = day9_1(&d);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day9_1};

    #[test]
    fn difference_test() {
        let v = vec![0, 2, 0, 3, 10];
        let mut diff = 0;
        for i in 1..v.len() {
            diff = v[i] - diff;
        }
        println!("{}", diff);
    }
    #[test]
    fn day9_test() {
        let d = advent_of_code::Reader::read_file("./input/day9_1_test.txt").unwrap();
        let result = day9_1(&d);
        println!("result: {result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn day9_final() {
        let d = advent_of_code::Reader::read_file("./input/day9_1.txt").unwrap();
        let result = day9_1(&d);
        assert_eq!(result, 864);
    }
}
