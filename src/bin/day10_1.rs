#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;
use std::ops::{Add, Sub};

fn day10_1(data: &Vec<String>) -> i32 {
    todo!()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    col: usize,
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Tile {
    item: char,
    position: Point,
    origin: Point,
}

impl Tile {
    fn from_points(origin: Point, position: Point, grid: &Vec<Vec<char>>) -> Tile {
        Tile {
            item: grid[position.row][position.col],
            position,
            origin,
        }
    }
    fn find_start_tile(grid: &Vec<Vec<char>>) -> Tile {
        let start_row = grid
            .iter()
            .enumerate()
            .filter_map(|(i, row)| if row.contains(&'S') { Some(i) } else { None })
            .nth(0)
            .unwrap();

        let row = grid[start_row].clone();
        let start_col = row
            .iter()
            .enumerate()
            .filter_map(|(j, &c)| if c == 'S' { Some(j) } else { None })
            .nth(0)
            .unwrap();

        Tile {
            item: 'S',
            position: Point {
                row: start_row,
                col: start_col,
            },
            origin: Point {
                row: start_row,
                col: start_row,
            },
        }
    }

    fn get_start_connections(&self, grid: &Vec<Vec<char>>, start_tile: &Tile) -> Vec<Tile> {
        let mut connections: Vec<Tile> = Vec::<Tile>::new();
        // valid connections:
        //    |
        // LF-S-7J
        //    |
        // check: North
        let check = Point {
            row: start_tile.position.row - 1,
            col: start_tile.position.col,
        };
        match grid[check.row][check.col] {
            '|' => connections.push(Tile {
                item: '|',
                position: check,
                origin: start_tile.position,
            }),
            _ => (),
        }

        // check: South
        let check = Point {
            row: start_tile.position.row + 1,
            col: start_tile.position.col,
        };
        match grid[check.row][check.col] {
            '|' => connections.push(Tile {
                item: '|',
                position: check,
                origin: start_tile.position,
            }),
            _ => (),
        }
        // check: West
        let check = Point {
            row: start_tile.position.row,
            col: start_tile.position.col - 1,
        };
        match grid[check.row][check.col] {
            '-' => connections.push(Tile {
                item: '-',
                position: check,
                origin: start_tile.position,
            }),
            'L' => connections.push(Tile {
                item: 'L',
                position: check,
                origin: start_tile.position,
            }),
            'F' => connections.push(Tile {
                item: 'F',
                position: check,
                origin: start_tile.position,
            }),
            _ => (),
        }
        // check: South
        let check = Point {
            row: start_tile.position.row,
            col: start_tile.position.col + 1,
        };
        match grid[check.row][check.col] {
            '-' => connections.push(Tile {
                item: '-',
                position: check,
                origin: start_tile.position,
            }),
            '7' => connections.push(Tile {
                item: '7',
                position: check,
                origin: start_tile.position,
            }),
            'J' => connections.push(Tile {
                item: 'J',
                position: check,
                origin: start_tile.position,
            }),
            _ => (),
        }
        connections
    }

    fn get_next_tile(&self, grid: &Vec<Vec<char>>) -> Option<Tile> {
        let mut p = self.position.clone();
        match self.item {
            '|' => {
                if self.origin.row < self.position.row {
                    p.row += 1;
                } else {
                    p.row -= 1;
                }
                return Some(Tile::from_points(self.position, p, grid));
            }
            'L' => {
                if self.origin.row < self.position.row {
                    p.col += 1;
                }
                if self.origin.col > self.position.col {
                    p.row -= 1;
                }
                return Some(Tile::from_points(self.position, p, grid));
            }
            '-' => {
                if self.origin.col < self.position.col {
                    p.col += 1;
                } else {
                    p.col -= 1;
                }
                return Some(Tile::from_points(self.position, p, grid));
            }
            'J' => {
                if self.origin.row < self.position.row {
                    p.col -= 1;
                }
                if self.origin.col < self.position.col {
                    p.row -= 1;
                }
                return Some(Tile::from_points(self.position, p, grid));
            }
            '7' => {
                if self.origin.row > self.position.row {
                    p.col -= 1;
                }
                if self.origin.col < self.position.col {
                    p.row += 1;
                }
                return Some(Tile::from_points(self.position, p, grid));
            }
            _ => None,
        }
    }
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day10_1.txt").unwrap();
    let result = day10_1(&d);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day10_1, Point, Tile};

    #[test]
    fn day10_find_start() {
        let d = advent_of_code::Reader::read_file("./input/day10_loop.txt").unwrap();
        let grid = d
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let start_row = grid
            .iter()
            .enumerate()
            .filter_map(|(i, row)| if row.contains(&'S') { Some(i) } else { None })
            .nth(0)
            .unwrap();
        // .collect::<Vec<usize>>();

        println!("{:?}", start_row);
        let row = grid[start_row].clone();
        let start_col = row
            .iter()
            .enumerate()
            .filter_map(|(j, &c)| if c == 'S' { Some(j) } else { None })
            .nth(0)
            .unwrap();

        let start_tile = Tile {
            item: 'S',
            position: Point {
                row: start_row,
                col: start_col,
            },
            origin: Point {
                row: start_row,
                col: start_row,
            },
        };

        println!("{start_tile:?}");
        assert_eq!(start_tile.position.row, 1);
        assert_eq!(start_tile.position.col, 1);
    }

    #[test]
    fn find_connection_to_start() {
        let d = advent_of_code::Reader::read_file("./input/day10_loop.txt").unwrap();
        let grid = d
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let start_tile = Tile {
            item: 'S',
            position: Point { row: 1, col: 1 },
            origin: Point { row: 0, col: 0 },
        };
        let mut connections: Vec<Tile> = Vec::<Tile>::new();
        // valid connections:
        //    |
        // LF-S-7J
        //    |
        // check: North
        let check = Point {
            row: start_tile.position.row - 1,
            col: start_tile.position.col,
        };
        match grid[check.row][check.col] {
            '|' => connections.push(Tile {
                item: '|',
                position: check,
                origin: start_tile.position,
            }),
            _ => (),
        }

        // check: South
        let check = Point {
            row: start_tile.position.row + 1,
            col: start_tile.position.col,
        };
        match grid[check.row][check.col] {
            '|' => connections.push(Tile {
                item: '|',
                position: check,
                origin: start_tile.position,
            }),
            _ => (),
        }
        // check: West
        let check = Point {
            row: start_tile.position.row,
            col: start_tile.position.col - 1,
        };
        match grid[check.row][check.col] {
            '-' => connections.push(Tile {
                item: '-',
                position: check,
                origin: start_tile.position,
            }),
            'L' => connections.push(Tile {
                item: 'L',
                position: check,
                origin: start_tile.position,
            }),
            'F' => connections.push(Tile {
                item: 'F',
                position: check,
                origin: start_tile.position,
            }),
            _ => (),
        }
        // check: South
        let check = Point {
            row: start_tile.position.row,
            col: start_tile.position.col + 1,
        };
        match grid[check.row][check.col] {
            '-' => connections.push(Tile {
                item: '-',
                position: check,
                origin: start_tile.position,
            }),
            '7' => connections.push(Tile {
                item: '7',
                position: check,
                origin: start_tile.position,
            }),
            'J' => connections.push(Tile {
                item: 'J',
                position: check,
                origin: start_tile.position,
            }),
            _ => (),
        }
        assert_eq!(connections.len(), 2);
        println!("{:?}", &connections);
    }

    #[test]
    fn walk_test() {
        let d = advent_of_code::Reader::read_file("./input/day10_loop.txt").unwrap();
        let grid = d
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let start_tile = Tile::find_start_tile(&grid);
        let connections = start_tile.get_start_connections(&grid, &start_tile);

        let mut path: Vec<Tile> = vec![start_tile, connections[1]];

        let next_tile = &path.iter().last().unwrap().get_next_tile(&grid).unwrap();
        path.push(next_tile.clone());
        // println!("{next_tile:?}");
        let next_tile = &path.iter().last().unwrap().get_next_tile(&grid).unwrap();
        path.push(*next_tile);
        let next_tile = &path.iter().last().unwrap().get_next_tile(&grid).unwrap();
        path.push(*next_tile);
        let next_tile = &path.iter().last().unwrap().get_next_tile(&grid).unwrap();
        path.push(*next_tile);
        let next_tile = &path.iter().last().unwrap().get_next_tile(&grid).unwrap();
        path.push(*next_tile);
        let next_tile = &path.iter().last().unwrap().get_next_tile(&grid).unwrap();
        path.push(*next_tile);
        let next_tile = &path.iter().last().unwrap().get_next_tile(&grid).unwrap();
        path.push(*next_tile);
        for t in &path {
            println!("{t:?}");
        }
    }
    #[test]
    fn day10_test() {
        let d = advent_of_code::Reader::read_file("./input/day10_1_test.txt").unwrap();
        let result = day10_1(&d);
        println!("result: {result}");
        assert_eq!(result, 114);
    }

    #[test]
    fn day10_final() {
        let d = advent_of_code::Reader::read_file("./input/day10_1.txt").unwrap();
        let result = day10_1(&d);
        assert_eq!(result, 1647269739);
    }
}
