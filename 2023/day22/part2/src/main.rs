use std::{
    collections::{HashMap, HashSet},
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

    let bricks = parse_bricks(reader);

    let stacked_bricks = stack_bricks(bricks);

    let count = count_falling_bricks(&stacked_bricks);

    println!("Count {count}");
}

#[derive(Debug)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    fn iter_xy(&self) -> impl Iterator<Item = (usize, usize)> {
        let min_x = std::cmp::min(self.start.0, self.end.0);
        let max_x = std::cmp::max(self.start.0, self.end.0);
        let min_y = std::cmp::min(self.start.1, self.end.1);
        let max_y = std::cmp::max(self.start.1, self.end.1);

        (min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| (x, y)))
    }

    fn height(&self) -> usize {
        let min_z = std::cmp::min(self.start.2, self.end.2);
        let max_z = std::cmp::max(self.start.2, self.end.2);

        max_z - min_z
    }

    fn xy_area(&self) -> Area {
        let min_x = std::cmp::min(self.start.0, self.end.0);
        let max_x = std::cmp::max(self.start.0, self.end.0);
        let min_y = std::cmp::min(self.start.1, self.end.1);
        let max_y = std::cmp::max(self.start.1, self.end.1);

        Area {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }
}

#[derive(Debug)]
struct Area {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Area {
    pub fn overlaps(self, other: &Self) -> bool {
        self.x + self.width >= other.x
            && self.y + self.height >= other.y
            && self.x <= other.x + other.width
            && self.y <= other.y + other.height
    }
}

fn count_falling_bricks(stacked_bricks: &[Brick]) -> usize {
    // (bottom z of block) -> Vec<(block idx, area occupied by that block)>
    let mut z_area_map: HashMap<usize, Vec<(usize, Area)>> = HashMap::new();

    // The bricks are already sorted by z asc
    for (idx, brick) in stacked_bricks.iter().enumerate() {
        let bottom_z = std::cmp::min(brick.start.2, brick.end.2);
        let area = brick.xy_area();

        if let Some(areas) = z_area_map.get_mut(&bottom_z) {
            areas.push((idx, area))
        } else {
            z_area_map.insert(bottom_z, vec![(idx, area)]);
        }
    }

    // (block idx) -> Vec<supported by block idx>
    let mut supporters_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for (idx, brick) in stacked_bricks.iter().enumerate() {
        let top_z = std::cmp::max(brick.start.2, brick.end.2);

        let blocks_resting_above = z_area_map.get(&(top_z + 1));

        if let Some(blocks_resting_at_top_z) = blocks_resting_above {
            for (block_resting_at_top_z_idx, block_resting_at_top_z_area) in blocks_resting_at_top_z
            {
                if brick.xy_area().overlaps(block_resting_at_top_z_area) {
                    if let Some(idxs) = supporters_map.get_mut(block_resting_at_top_z_idx) {
                        idxs.push(idx);
                    } else {
                        supporters_map.insert(*block_resting_at_top_z_idx, vec![idx]);
                    }
                }
            }
        }
    }

    // (block idx) -> Vec<is supporting block idx>
    let mut supporters_map_rev: HashMap<usize, Vec<usize>> = HashMap::new();

    for (b_idx, supported_by_idxs) in supporters_map.iter() {
        for supported_by_idx in supported_by_idxs {
            if let Some(idxs) = supporters_map_rev.get_mut(supported_by_idx) {
                idxs.push(*b_idx);
            } else {
                supporters_map_rev.insert(*supported_by_idx, vec![*b_idx]);
            }
        }
    }

    let mut count = 0;

    for brick_idx in 0..stacked_bricks.len() {
        let mut falling_bricks: HashSet<usize> = HashSet::from_iter([brick_idx]);
        let mut bricks_to_walk = vec![brick_idx];

        while let Some(b) = bricks_to_walk.pop() {
            let bricks_supported = supporters_map_rev.get(&b);

            if let Some(bricks_supported) = bricks_supported {
                for brick_supported in bricks_supported {
                    let bricks_supported_by_current = supporters_map.get(brick_supported).unwrap();

                    if bricks_supported_by_current
                        .iter()
                        .all(|b| falling_bricks.contains(b))
                    {
                        bricks_to_walk.push(*brick_supported);
                        falling_bricks.insert(*brick_supported);
                    }
                }
            }
        }

        count += falling_bricks.len() - 1;
    }

    count
}

fn stack_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    // (x, y) -> (stacked z)
    let mut z_map = HashMap::with_capacity(bricks.len());

    // Sort by z asc
    bricks
        .sort_by(|a, b| std::cmp::min(a.start.2, a.end.2).cmp(&std::cmp::min(b.start.2, b.end.2)));

    for brick in bricks.iter_mut() {
        let max_exisiting_z = brick
            .iter_xy()
            .map(|xy| *z_map.get(&xy).unwrap_or(&0))
            .max()
            .unwrap_or(0);

        let new_z: usize = max_exisiting_z + 1;
        let z_offset = std::cmp::min(brick.start.2, brick.end.2) - new_z;

        brick.start.2 -= z_offset;
        brick.end.2 -= z_offset;

        let brick_height = brick.height();

        for xy in brick.iter_xy() {
            z_map.insert(xy, new_z + brick_height);
        }
    }

    bricks
}

fn parse_bricks(reader: BufReader<File>) -> Vec<Brick> {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read line");

            let (start, end) = line.split_once('~').expect("Line split failed");

            let start = start
                .split(',')
                .map(|n| n.parse().expect("Num parse failed"))
                .collect::<Vec<_>>();
            let end = end
                .split(',')
                .map(|n| n.parse().expect("Num parse failed"))
                .collect::<Vec<_>>();

            Brick {
                start: (start[0], start[1], start[2]),
                end: (end[0], end[1], end[2]),
            }
        })
        .collect()
}
