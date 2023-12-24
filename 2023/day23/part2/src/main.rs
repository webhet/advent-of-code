use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

// Brute force. Takes ~30min :(
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
    Block,
}

fn longest_path(
    adj_map: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut dist = adj_map
        .keys()
        .map(|k| (*k, 0_isize))
        .collect::<HashMap<_, _>>();
    let mut visited = HashSet::new();

    lp(start, 0, &mut dist, &mut visited, adj_map);

    let end_cost = *dist.get(&end).unwrap();

    if end_cost != 0 {
        Some(end_cost as usize)
    } else {
        None
    }
}

fn lp(
    node: (usize, usize),
    cost: isize,
    dist: &mut HashMap<(usize, usize), isize>,
    visited: &mut HashSet<(usize, usize)>,
    adj_map: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);

    if *dist.get(&node).unwrap() < cost {
        *dist.get_mut(&node).unwrap() = cost;
    }

    for edge in adj_map.get(&node).unwrap() {
        lp(*edge, cost + 1, dist, visited, adj_map);
    }

    visited.remove(&node);
}

fn map_to_adj_graph(
    start: (usize, usize),
    end: (usize, usize),
    map: &[Vec<Tile>],
) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
    let max_x = map.first().map(|r| r.len() - 1).unwrap_or(0);
    let max_y = map.len() - 1;
    let mut adj_map: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut visited = HashSet::new();

    let mut tiles_to_examine = vec![start];

    while let Some(tile) = tiles_to_examine.pop() {
        visited.insert(tile);

        let neighbours = get_neighbours_for_tile(tile, max_x, max_y, map);

        for neighbour in neighbours {
            if let Some(coords) = adj_map.get_mut(&tile) {
                coords.insert(neighbour);
            } else {
                adj_map.insert(tile, HashSet::from_iter([neighbour]));
            }

            if let Some(coords) = adj_map.get_mut(&neighbour) {
                coords.insert(tile);
            } else {
                adj_map.insert(neighbour, HashSet::from_iter([tile]));
            }

            if !visited.contains(&neighbour) {
                tiles_to_examine.push(neighbour);
            }
        }
    }

    adj_map.insert(end, HashSet::new());

    adj_map
}

fn get_neighbours_for_tile(
    (x, y): (usize, usize),
    max_x: usize,
    max_y: usize,
    map: &[Vec<Tile>],
) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    // Up
    if y > 0 && matches!(map[y - 1][x], Tile::Path) {
        neighbours.push((x, y - 1));
    }

    // Down
    if y < max_y && matches!(map[y + 1][x], Tile::Path) {
        neighbours.push((x, y + 1));
    }

    // Left
    if x > 0 && matches!(map[y][x - 1], Tile::Path) {
        neighbours.push((x - 1, y));
    }

    // Right
    if x < max_x && matches!(map[y][x + 1], Tile::Path) {
        neighbours.push((x + 1, y));
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
                    '^' => Tile::Path,
                    'v' => Tile::Path,
                    '<' => Tile::Path,
                    '>' => Tile::Path,
                    _ => panic!("Unexpected tile"),
                })
                .collect()
        })
        .collect()
}
