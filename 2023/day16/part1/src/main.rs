use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part1 <filepath>");
        return;
    };

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening input file: {err}");
            return;
        }
    };

    let reader = BufReader::new(file);

    let tiles = parse_tiles(reader);

    let row_len = tiles.first().map(|v| v.len()).unwrap_or(0);

    let mut markings = std::iter::repeat(vec![HashSet::new(); row_len])
        .take(tiles.len())
        .collect::<Vec<_>>();

    mark_grid(&tiles, &mut markings, row_len);

    let tiles_marked: usize =
        markings
            .iter()
            .flatten()
            .fold(0, |acc, v| if !v.is_empty() { acc + 1 } else { acc });

    println!("Tiles marked {tiles_marked}");
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn step(&self, x: usize, y: usize, max_x: usize, max_y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if y < max_y {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Direction::Right => {
                if x < max_x {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    MirrorSlash,
    MirrorBackSlash,
    Vertical,
    Horizontal,
    Empty,
}

impl Tile {
    pub fn out_directions(&self, in_direction: Direction) -> Vec<Direction> {
        match self {
            Tile::MirrorSlash => match in_direction {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            Tile::MirrorBackSlash => match in_direction {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            Tile::Vertical => match in_direction {
                Direction::Up => vec![Direction::Up],
                Direction::Down => vec![Direction::Down],
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Right => vec![Direction::Up, Direction::Down],
            },
            Tile::Horizontal => match in_direction {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Left],
                Direction::Right => vec![Direction::Right],
            },
            Tile::Empty => vec![in_direction],
        }
    }
}

fn mark_grid(tiles: &[Vec<Tile>], markings: &mut [Vec<HashSet<Direction>>], row_len: usize) {
    mark_pos(0, 0, Direction::Right, tiles, markings, row_len);
}

fn mark_pos(
    x: usize,
    y: usize,
    direction: Direction,
    tiles: &[Vec<Tile>],
    markings: &mut [Vec<HashSet<Direction>>],
    row_len: usize,
) {
    if !markings[y][x].insert(direction) {
        return;
    }

    let out_dirs = tiles[y][x].out_directions(direction);

    for out_dir in out_dirs {
        if let Some((new_x, new_y)) = out_dir.step(x, y, row_len - 1, tiles.len() - 1) {
            mark_pos(new_x, new_y, out_dir, tiles, markings, row_len);
        }
    }
}

fn parse_tiles(reader: BufReader<File>) -> Vec<Vec<Tile>> {
    let mut tiles = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let line_tiles = line
            .chars()
            .map(|c| match c {
                '/' => Tile::MirrorSlash,
                '\\' => Tile::MirrorBackSlash,
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                '.' => Tile::Empty,
                _ => panic!("Unexpected tile"),
            })
            .collect();

        tiles.push(line_tiles);
    }

    tiles
}
