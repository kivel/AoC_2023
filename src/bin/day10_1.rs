#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day10_1(data: &Vec<String>) -> i32 {
    todo!()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    col: usize,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Tile {
    item: char,
    position: Point,
    origin: Point,
}

impl Tile {
    fn get_start_connections(&self, grid: Vec<Vec<char>>, start_tile: &Tile) -> Vec<Tile> {
        // let d = advent_of_code::Reader::read_file("./input/day10_loop.txt").unwrap();
        // let grid = d
        //     .iter()
        //     .map(|line| line.chars().collect::<Vec<char>>())
        //     .collect::<Vec<Vec<char>>>();

        // let start_tile = Tile {
        //     item: 'S',
        //     position: Point { row: 1, col: 1 },
        //     origin: Point { row: 0, col: 0 },
        // };
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
        connections
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

        println!("grid: {:?}", &grid);
        println!("start: {:?}", &start_tile);
        let connections = start_tile.get_start_connections(grid, &start_tile);
        println!("connections: {:?}", &connections);
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
