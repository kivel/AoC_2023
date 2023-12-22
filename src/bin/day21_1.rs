use std::collections::{HashMap, HashSet};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn get_neighbors(grid: &Vec<Vec<char>>, position: (usize, usize)) -> HashSet<(usize, usize)> {
    let row = position.0;
    let col = position.1;
    let mut result = HashSet::new();

    let nesw: Vec<(usize, usize)> = vec![
        (row, (col + 1).min(grid[0].len() - 1)),
        (row, col.saturating_sub(1)),
        ((row + 1).min(grid.len() - 1), col),
        (row.saturating_sub(1), col),
    ];

    nesw.iter().for_each(|p| {
        if grid[p.0][p.1] == '.' {
            result.insert(p.clone());
        }
    });

    result.remove(&position); // in case nesw was clamped onto position

    result
}

#[allow(dead_code)]
fn plot_map(grid: &Vec<Vec<char>>, start: &(usize, usize), visited: &HashSet<(usize, usize)>) {
    let mut map = grid.clone();

    visited.iter().for_each(|g| map[g.0][g.1] = 'O');
    map[start.0][start.1] = 'S';

    for (i, r) in map.iter().enumerate() {
        print!("{i:02}: ");
        for c in r {
            print!("{c}");
        }
        println!();
    }
}
fn day21_1(data: &str, n_steps: usize) -> usize {
    // keep track of visited positions and their respective neighbors
    let mut good: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    let mut start = (0usize, 0usize);
    let grid = data
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = (i, j);
                        return '.'; // remove 'S'
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    println!("START: {:?}", start);

    let mut steps: HashSet<(usize, usize)> = HashSet::new();
    steps.insert(start);

    println!("GOOD: {:?}", &good);

    for _ in 0..n_steps {
        let mut s: HashSet<(usize, usize)> = HashSet::new();
        steps.iter().for_each(|step| {
            s.extend(
                good.entry(*step)
                    .or_insert(get_neighbors(&grid, *step))
                    .clone(),
            );
        });
        steps = s;
    }

    return steps.len();
}

fn main() {
    let d = include_str!("../../input/day21.txt");
    let result = day21_1(d, 64);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::day21_1;

    #[test]
    fn day21_1_test() {
        let d = include_str!("../../input/day21_test.txt");
        let result = day21_1(d, 6);
        println!("{result:?}");
        assert_eq!(result, 16)
    }

    #[test]
    fn day21_1_final() {
        let d = include_str!("../../input/day21.txt");
        let result = day21_1(d, 64);
        println!("{}", result);
        assert_eq!(result, 3572);
    }
}
