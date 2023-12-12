use std::cmp::{max, min};
use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day11_2(data: &Vec<String>, expand: usize) -> usize {
    let grid = data
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut universe = Universe { space: grid };

    let galaxy_pairs = universe.find_galaxy_pairs();
    let empty_space = universe.get_empty_space();

    galaxy_pairs
        .iter()
        .map(|pair| get_galaxy_distance(pair, &empty_space, expand))
        // .map(|pair| pair.1.abs_diff(pair.3) + pair.0.abs_diff(pair.2))
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>()
}

fn get_galaxy_distance(
    pair: &(usize, usize, usize, usize),
    empty_space: &(Vec<usize>, Vec<usize>),
    expand: usize,
) -> usize {
    let row_size = &pair.0.abs_diff(pair.2);
    let col_size = &pair.1.abs_diff(pair.3);
    let row_range = min(pair.0, pair.2)..=max(pair.0, pair.2);
    let col_range = min(pair.1, pair.3)..=max(pair.1, pair.3);

    let row_count = empty_space
        .0
        .iter()
        .filter(|&x| row_range.contains(x))
        .count();
    let col_count = empty_space
        .1
        .iter()
        .filter(|&x| col_range.contains(x))
        .count();

    (row_size - row_count) + row_count * expand + (col_size - col_count) + col_count * expand
}
struct Universe {
    space: Vec<Vec<char>>,
}

impl Universe {
    #[allow(dead_code)]
    fn show(&self) {
        for row in &self.space {
            for c in row {
                print!("{c}");
            }
            println!();
        }
    }

    fn get_empty_space(&self) -> (Vec<usize>, Vec<usize>) {
        let space = self.space.clone();
        let empty_rows: Vec<usize> = space
            .iter()
            .enumerate()
            .filter_map(|(i, line)| {
                if line.iter().all(|c| *c == '.') {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        let empty_cols: Vec<usize> = Self::transpose(space)
            .iter()
            .enumerate()
            .filter_map(|(i, line)| {
                if line.iter().all(|c| *c == '.') {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        (empty_rows, empty_cols)
    }

    fn transpose(space: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let rows = space.len();
        let cols = space[0].len();
        let mut transposed_matrix: Vec<Vec<char>> = vec![vec!['.'; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                transposed_matrix[j][i] = space[i][j];
            }
        }

        transposed_matrix
    }

    fn expand(&mut self) {
        let empty_row: Vec<char> = vec!['.'; self.space[0].len()];
        let mut insert_position: Vec<usize> = Vec::new();
        for (i, row) in self.space.iter().enumerate() {
            if row.iter().all(|c| *c == '.') {
                insert_position.push(i + insert_position.len())
            }
        }
        let mut new_space: Vec<Vec<char>> = self.space.clone();
        for i in insert_position {
            new_space.insert(i, empty_row.clone());
        }

        self.space = new_space;
    }

    fn find_galaxy_pairs(&self) -> HashSet<(usize, usize, usize, usize)> {
        let mut hash_positions: Vec<(usize, usize)> = Vec::new();

        // Find all '#' positions in space
        for (i, row) in self.space.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == '#' {
                    hash_positions.push((i, j));
                }
            }
        }

        let mut galaxy_pairs: HashSet<(usize, usize, usize, usize)> = HashSet::new();

        // Find unique pairs of galaxy positions
        for i in 0..hash_positions.len() {
            for j in i + 1..hash_positions.len() {
                let pair = (
                    hash_positions[i].0,
                    hash_positions[i].1,
                    hash_positions[j].0,
                    hash_positions[j].1,
                );
                galaxy_pairs.insert(pair);
            }
        }

        galaxy_pairs
    }
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day11_1.txt").unwrap();

    let result = day11_2(&d, 1_000_000);
    println!("{result:?}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day11_2};

    #[test]
    fn day11_test_expand2() {
        let d = advent_of_code::Reader::read_file("./input/day11_1_test.txt").unwrap();
        let result = day11_2(&d, 2);
        assert_eq!(result, 374);
    }
    #[test]
    fn day11_test_expand10() {
        let d = advent_of_code::Reader::read_file("./input/day11_1_test.txt").unwrap();
        let result = day11_2(&d, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn day11_test_expand100() {
        let d = advent_of_code::Reader::read_file("./input/day11_1_test.txt").unwrap();
        let result = day11_2(&d, 100);
        assert_eq!(result, 8410);
    }
    #[test]
    fn day11_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day11_1.txt").unwrap();
        let result = day11_2(&d, 1_000_000);
        assert_eq!(result, 632003913611);
    }
}
