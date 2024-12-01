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

    let hailstones = parse_hailstones(reader);

    let collision_points = get_collisions_of_pairs(&hailstones);

    let count = collision_points
        .iter()
        .filter(|(x, y)| {
            *x >= 200000000000000_f64
                && *x <= 400000000000000_f64
                && *y >= 200000000000000_f64
                && *y <= 400000000000000_f64
        })
        .count();

    println!("Count: {count}");
}

#[derive(Debug, PartialEq, Eq)]
struct Hailstone {
    pos: (i128, i128, i128),
    vel: (i128, i128, i128),
}

fn get_collisions_of_pairs(hailstones: &[Hailstone]) -> Vec<(f64, f64)> {
    let mut collisions = Vec::new();

    for (offset, a) in hailstones.iter().enumerate() {
        for b in hailstones[(offset + 1)..].iter() {
            if let Some(collision_point) = get_xy_collision_point_in_future(a, b) {
                collisions.push(collision_point);
            }
        }
    }

    collisions
}

fn get_xy_collision_point_in_future(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    let x1 = a.pos.0;
    let y1 = a.pos.1;
    let x2 = x1 + a.vel.0;
    let y2 = y1 + a.vel.1;

    let x3 = b.pos.0;
    let y3 = b.pos.1;
    let x4 = x3 + b.vel.0;
    let y4 = y3 + b.vel.1;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if denom == 0 {
        return None;
    }

    let px_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
    let py_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

    let collision_point = (px_num as f64 / denom as f64, py_num as f64 / denom as f64);

    // They only cross in the future if both hailstons fly towards the collision point.

    let a_to_coll = (
        collision_point.0 - a.pos.0 as f64,
        collision_point.1 - a.pos.1 as f64,
    );
    let b_to_coll = (
        collision_point.0 - b.pos.0 as f64,
        collision_point.1 - b.pos.1 as f64,
    );

    let both_fly_towards_coll = a_to_coll.0.signum() as i128 == a.vel.0.signum()
        && a_to_coll.1.signum() as i128 == a.vel.1.signum()
        && b_to_coll.0.signum() as i128 == b.vel.0.signum()
        && b_to_coll.1.signum() as i128 == b.vel.1.signum();

    if both_fly_towards_coll {
        Some(collision_point)
    } else {
        None
    }
}

fn parse_hailstones(reader: BufReader<File>) -> Vec<Hailstone> {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read line");

            let (pos, vel) = line.split_once(" @ ").expect("Line split failed");

            let pos = pos
                .split(',')
                .map(|num| num.trim().parse().expect("Num parse failed"))
                .collect::<Vec<_>>();

            let vel = vel
                .split(',')
                .map(|num| num.trim().parse().expect("Num parse failed"))
                .collect::<Vec<_>>();

            Hailstone {
                pos: (pos[0], pos[1], pos[2]),
                vel: (vel[0], vel[1], vel[2]),
            }
        })
        .collect()
}
