use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
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

    let map = parse_map(reader);

    let shortest_path = shortest_path(&map).expect("No path found");

    println!("Shortest path {shortest_path}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn neighbours(
        &self,
        x: usize,
        y: usize,
        max_x: usize,
        max_y: usize,
    ) -> Vec<((usize, usize), Direction)> {
        match self {
            Direction::Up => {
                let mut v = Vec::new();

                if x > 0 {
                    v.push(((x - 1, y), Direction::Left));
                }
                if x < max_x {
                    v.push(((x + 1, y), Direction::Right));
                }
                if y > 0 {
                    v.push(((x, y - 1), Direction::Up));
                }

                v
            }
            Direction::Down => {
                let mut v = Vec::new();

                if x > 0 {
                    v.push(((x - 1, y), Direction::Left));
                }
                if x < max_x {
                    v.push(((x + 1, y), Direction::Right));
                }
                if y < max_y {
                    v.push(((x, y + 1), Direction::Down));
                }

                v
            }
            Direction::Left => {
                let mut v = Vec::new();

                if y > 0 {
                    v.push(((x, y - 1), Direction::Up));
                }
                if y < max_y {
                    v.push(((x, y + 1), Direction::Down));
                }
                if x > 0 {
                    v.push(((x - 1, y), Direction::Left));
                }

                v
            }
            Direction::Right => {
                let mut v = Vec::new();

                if y > 0 {
                    v.push(((x, y - 1), Direction::Up));
                }
                if y < max_y {
                    v.push(((x, y + 1), Direction::Down));
                }
                if x < max_x {
                    v.push(((x + 1, y), Direction::Right));
                }

                v
            }
        }
    }

    pub fn steps_left(&self, x: usize, y: usize, max_x: usize, max_y: usize) -> usize {
        match self {
            Direction::Up => y,
            Direction::Down => max_y - y,
            Direction::Left => x,
            Direction::Right => max_x - x,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    heatloss: u32,
    x: usize,
    y: usize,
    direction: Option<Direction>,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heatloss.cmp(&self.heatloss)
        //.then_with(|| (self.y * 12 + self.x).cmp(&(other.y * 12 + other.x)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(map: &[Vec<u32>]) -> Option<u32> {
    let row_len = map.first().map(|r| r.len()).unwrap_or(0);

    let mut heap = BinaryHeap::new();

    let mut seen = HashSet::new();

    heap.push(State {
        heatloss: 0,
        x: 0,
        y: 0,
        direction: None,
        steps: 0,
    });

    let max_x = row_len - 1;
    let max_y = map.len() - 1;

    while let Some(State {
        heatloss,
        x,
        y,
        direction,
        steps,
    }) = heap.pop()
    {
        if x == max_x && y == max_y {
            return Some(heatloss);
        }

        let seen_state = (x, y, direction, steps);
        if seen.contains(&seen_state) {
            continue;
        }

        seen.insert(seen_state);

        let neighbours = direction
            .unwrap_or(Direction::Right)
            .neighbours(x, y, max_x, max_y);

        for neighbour in neighbours {
            if direction == Some(neighbour.1) {
                // Straight
                if steps < 10 {
                    heap.push(State {
                        heatloss: heatloss + map[neighbour.0 .1][neighbour.0 .0],
                        x: neighbour.0 .0,
                        y: neighbour.0 .1,
                        direction: Some(neighbour.1),
                        steps: steps + 1,
                    });
                }
            } else if (steps >= 4 && neighbour.1.steps_left(x, y, max_x, max_y) >= 4)
                || direction.is_none()
            {
                // Turning
                heap.push(State {
                    heatloss: heatloss + map[neighbour.0 .1][neighbour.0 .0],
                    x: neighbour.0 .0,
                    y: neighbour.0 .1,
                    direction: Some(neighbour.1),
                    steps: 1,
                });
            }
        }
    }

    None
}

fn parse_map(reader: BufReader<File>) -> Vec<Vec<u32>> {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read line");

            line.chars()
                .map(|c| c.to_digit(10).expect("Num parse failed"))
                .collect()
        })
        .collect()
}
