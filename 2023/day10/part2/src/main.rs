use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let Some(filepath) = env::args().nth(1) else {
        println!("Usage: part2 <filepath>");
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

    let tile_grid = parse_tile_grid(reader);

    let marked_grid = map_to_marked_grid(&tile_grid);

    let tiles_enclosed = count_enclosed_tiles(&marked_grid);

    println!("Tiles enclosed: {tiles_enclosed}");
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn step(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::East => Some((x + 1, y)),
            Direction::South => Some((x, y + 1)),
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::Vertical),
            '-' => Ok(Tile::Horizontal),
            'L' => Ok(Tile::NorthEast),
            'J' => Ok(Tile::NorthWest),
            '7' => Ok(Tile::SouthWest),
            'F' => Ok(Tile::SouthEast),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err(()),
        }
    }
}

impl Tile {
    pub fn out_direction(&self, in_direction: Direction) -> Option<Direction> {
        match self {
            Tile::Vertical => match in_direction {
                Direction::North => Some(Direction::North),
                Direction::East => None,
                Direction::South => Some(Direction::South),
                Direction::West => None,
            },
            Tile::Horizontal => match in_direction {
                Direction::North => None,
                Direction::East => Some(Direction::East),
                Direction::South => None,
                Direction::West => Some(Direction::West),
            },
            Tile::NorthEast => match in_direction {
                Direction::North => None,
                Direction::East => None,
                Direction::South => Some(Direction::East),
                Direction::West => Some(Direction::North),
            },
            Tile::NorthWest => match in_direction {
                Direction::North => None,
                Direction::East => Some(Direction::North),
                Direction::South => Some(Direction::West),
                Direction::West => None,
            },
            Tile::SouthWest => match in_direction {
                Direction::North => Some(Direction::West),
                Direction::East => Some(Direction::South),
                Direction::South => None,
                Direction::West => None,
            },
            Tile::SouthEast => match in_direction {
                Direction::North => Some(Direction::East),
                Direction::East => None,
                Direction::South => None,
                Direction::West => Some(Direction::South),
            },
            Tile::Ground => match in_direction {
                Direction::North => None,
                Direction::East => None,
                Direction::South => None,
                Direction::West => None,
            },
            Tile::Start => match in_direction {
                Direction::North => None,
                Direction::East => None,
                Direction::South => None,
                Direction::West => None,
            },
        }
    }

    pub fn grid_marking(&self) -> GridMarking {
        match self {
            Tile::Vertical => GridMarking::Vertical,
            Tile::Horizontal => GridMarking::Horizontal,
            Tile::NorthEast => GridMarking::CornerUp,
            Tile::NorthWest => GridMarking::CornerUp,
            Tile::SouthWest => GridMarking::CornerDown,
            Tile::SouthEast => GridMarking::CornerDown,
            Tile::Ground => GridMarking::None,
            Tile::Start => GridMarking::None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum GridMarking {
    None,
    Horizontal,
    Vertical,
    CornerUp,
    CornerDown,
}

impl GridMarking {
    pub fn crosses(&self, dir: Direction) -> bool {
        match self {
            GridMarking::None => false,
            GridMarking::Horizontal => matches!(dir, Direction::North | Direction::South),
            GridMarking::Vertical => matches!(dir, Direction::East | Direction::West),
            GridMarking::CornerUp => false,
            GridMarking::CornerDown => false,
        }
    }
}

fn count_enclosed_tiles(marked_grid: &[Vec<GridMarking>]) -> usize {
    let mut count = 0;

    let max_x = marked_grid.first().map(|l| l.len() - 1).unwrap_or(0);

    for row in marked_grid.iter() {
        for x in 0..=max_x {
            if matches!(row[x], GridMarking::None) {
                let mut corners_up: usize = 0;
                let mut corners_down: usize = 0;

                let crossings: usize = (0..=x).fold(0, |acc, x_pos| {
                    match row[x_pos] {
                        GridMarking::CornerUp => {
                            corners_up += 1;
                        }
                        GridMarking::CornerDown => {
                            corners_down += 1;
                        }
                        _ => {}
                    }

                    if row[x_pos].crosses(Direction::West) {
                        acc + 1
                    } else {
                        acc
                    }
                });

                let corner_crossings = std::cmp::min(corners_up, corners_down);

                if (crossings + corner_crossings) % 2 == 1 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn map_to_marked_grid(tile_grid: &[Vec<Tile>]) -> Vec<Vec<GridMarking>> {
    let mut marked_grid: Vec<Vec<GridMarking>> = tile_grid
        .iter()
        .map(|line| vec![GridMarking::None; line.len()])
        .collect();

    let (mut x, mut y) = find_start(tile_grid).expect("No start found");
    let mut direction = find_start_direction(tile_grid, (x, y)).expect("No start direction found");
    let start_direction = direction;

    loop {
        let (new_x, new_y) = direction.step(x, y).expect("Failed to step in direction");
        x = new_x;
        y = new_y;

        let tile = tile_grid[new_y][new_x];

        if matches!(tile, Tile::Start) {
            let start_marking = match (start_direction, direction) {
                (Direction::North, Direction::North) | (Direction::South, Direction::South) => {
                    GridMarking::Vertical
                }
                (Direction::East, Direction::East) | (Direction::West, Direction::West) => {
                    GridMarking::Horizontal
                }
                (Direction::North, Direction::West)
                | (Direction::West, Direction::North)
                | (Direction::North, Direction::East)
                | (Direction::East, Direction::North) => GridMarking::CornerUp,
                (Direction::South, Direction::West)
                | (Direction::West, Direction::South)
                | (Direction::South, Direction::East)
                | (Direction::East, Direction::South) => GridMarking::CornerDown,
                _ => unreachable!(),
            };
            marked_grid[new_y][new_x] = start_marking;

            break;
        }

        marked_grid[new_y][new_x] = tile.grid_marking();

        direction = tile.out_direction(direction).expect("Can't walk that way");
    }

    marked_grid
}

fn find_start_direction(tile_grid: &[Vec<Tile>], start: (usize, usize)) -> Option<Direction> {
    const DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    for direction in DIRECTIONS {
        if let Some((x, y)) = direction.step(start.0, start.1) {
            let tile = tile_grid[y][x];

            if tile.out_direction(direction).is_some() {
                return Some(direction);
            }
        }
    }

    None
}

fn find_start(tile_grid: &[Vec<Tile>]) -> Option<(usize, usize)> {
    let mut x_pos = None;

    let y = tile_grid.iter().position(|tiles| {
        let x = tiles.iter().position(|t| matches!(t, Tile::Start));

        x_pos = x;

        x.is_some()
    });

    match (x_pos, y) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

fn parse_tile_grid(reader: BufReader<File>) -> Vec<Vec<Tile>> {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read line");

            line.chars()
                .map(|c| Tile::try_from(c).expect("Unexpected tile char"))
                .collect()
        })
        .collect()
}
