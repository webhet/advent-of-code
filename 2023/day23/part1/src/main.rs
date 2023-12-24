use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
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

    let map = parse_map(reader);
    let row_len = map.first().map(|r| r.len()).unwrap_or(0);

    let start = (1, 0);
    let end = (row_len - 2, map.len() - 1);

    let adj_map = map_to_adj_graph(start, end, &map);

    let steps = longest_path(&adj_map, start, end).expect("No path found");

    println!("Steps: {steps}");
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    SlopeUp,
    SlopeDown,
    SlopeRight,
    SlopeLeft,
    Block,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn longest_path(
    adj_map: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut dist = adj_map
        .keys()
        .map(|k| (*k, isize::MAX))
        .collect::<HashMap<_, _>>();

    let mut heap = BinaryHeap::new();

    *dist.get_mut(&start).unwrap() = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > *dist.get(&position).unwrap() {
            continue;
        }

        for edge in adj_map.get(&position).unwrap() {
            let next = State {
                cost: cost - 1,
                position: *edge,
            };

            if next.cost < *dist.get(&next.position).unwrap() {
                heap.push(next);
                *dist.get_mut(&next.position).unwrap() = next.cost;
            }
        }
    }

    let end_cost = *dist.get(&end).unwrap();

    // We can't early stop inside the loop as dijkstra isn't guaranteed to
    // find the longest path on a weight-reversed DAG.
    // So instead walk every node in the graph and then look for the cost of the end node.
    if end_cost != isize::MAX {
        Some((-end_cost) as usize)
    } else {
        None
    }
}

fn map_to_adj_graph(
    start: (usize, usize),
    end: (usize, usize),
    map: &[Vec<Tile>],
) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
    let max_x = map.first().map(|r| r.len() - 1).unwrap_or(0);
    let max_y = map.len() - 1;

    let mut adj_map: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    let mut tiles_to_examine = vec![(start, Direction::Down)];

    while let Some((tile, dir)) = tiles_to_examine.pop() {
        let neighbours = get_neighbours_for_tile(tile, max_x, max_y, dir, map);

        for neighbour in neighbours {
            if let Some(coords) = adj_map.get_mut(&tile) {
                coords.insert(neighbour.0);
            } else {
                adj_map.insert(tile, HashSet::from_iter([neighbour.0]));
            }

            tiles_to_examine.push(neighbour);
        }
    }

    adj_map.insert(end, HashSet::new());

    adj_map
}

fn get_neighbours_for_tile(
    (x, y): (usize, usize),
    max_x: usize,
    max_y: usize,
    dir: Direction,
    map: &[Vec<Tile>],
) -> Vec<((usize, usize), Direction)> {
    match map[y][x] {
        Tile::Path => {}
        Tile::SlopeUp => {
            return vec![((x, y - 1), Direction::Up)];
        }
        Tile::SlopeDown => {
            return vec![((x, y + 1), Direction::Down)];
        }
        Tile::SlopeRight => {
            return vec![((x + 1, y), Direction::Right)];
        }
        Tile::SlopeLeft => {
            return vec![((x - 1, y), Direction::Left)];
        }
        Tile::Block => unreachable!(),
    }

    let mut neighbours = Vec::new();

    // Up
    if y > 0
        && !matches!(dir, Direction::Down)
        && matches!(
            map[y - 1][x],
            Tile::Path | Tile::SlopeUp | Tile::SlopeRight | Tile::SlopeLeft
        )
    {
        neighbours.push(((x, y - 1), Direction::Up));
    }

    // Down
    if y < max_y
        && !matches!(dir, Direction::Up)
        && matches!(
            map[y + 1][x],
            Tile::Path | Tile::SlopeDown | Tile::SlopeRight | Tile::SlopeLeft
        )
    {
        neighbours.push(((x, y + 1), Direction::Down));
    }

    // Left
    if x > 0
        && !matches!(dir, Direction::Right)
        && matches!(
            map[y][x - 1],
            Tile::Path | Tile::SlopeUp | Tile::SlopeDown | Tile::SlopeLeft
        )
    {
        neighbours.push(((x - 1, y), Direction::Left));
    }

    // Right
    if x < max_x
        && !matches!(dir, Direction::Left)
        && matches!(
            map[y][x + 1],
            Tile::Path | Tile::SlopeUp | Tile::SlopeDown | Tile::SlopeRight
        )
    {
        neighbours.push(((x + 1, y), Direction::Right));
    }

    neighbours
}

fn parse_map(reader: BufReader<File>) -> Vec<Vec<Tile>> {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read line");

            line.chars()
                .map(|c| match c {
                    '.' => Tile::Path,
                    '#' => Tile::Block,
                    '^' => Tile::SlopeUp,
                    'v' => Tile::SlopeDown,
                    '<' => Tile::SlopeLeft,
                    '>' => Tile::SlopeRight,
                    _ => panic!("Unexpected tile"),
                })
                .collect()
        })
        .collect()
}
