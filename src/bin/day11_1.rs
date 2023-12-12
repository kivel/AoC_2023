use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day11_1(data: &Vec<String>) -> usize {
    let grid = data
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut universe = Universe { space: grid };

    universe.expand();
    universe.transpose();
    universe.expand();
    universe.transpose();

    let galaxy_pairs = universe.find_galaxy_pairs();

    galaxy_pairs
        .iter()
        .map(|pair| pair.1.abs_diff(pair.3) + pair.0.abs_diff(pair.2))
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>()
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

    fn transpose(&mut self) {
        let rows = self.space.len();
        let cols = self.space[0].len();
        let mut transposed_matrix: Vec<Vec<char>> = vec![vec!['.'; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                transposed_matrix[j][i] = self.space[i][j];
            }
        }

        self.space = transposed_matrix
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

    let result = day11_1(&d);
    println!("{result:?}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day11_1};

    #[test]
    fn day11_test() {
        let d = advent_of_code::Reader::read_file("./input/day11_1_test.txt").unwrap();
        let result = day11_1(&d);
        assert_eq!(result, 374);
    }

    #[test]
    fn day11_final() {
        let d = advent_of_code::Reader::read_file("./input/day11_1.txt").unwrap();
        let result = day11_1(&d);
        assert_eq!(result, 9177603);
    }
}
