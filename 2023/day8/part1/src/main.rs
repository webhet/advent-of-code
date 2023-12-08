use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
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
    let mut lines = reader.lines();

    let directions = parse_directions(&mut lines);
    // Consume empty line
    lines.next().expect("Unexpected EOF").ok();

    let jump_map = parse_map(&mut lines);

    let start = JumpPosition::try_from("AAA").unwrap();
    let end = JumpPosition::try_from("ZZZ").unwrap();

    let steps = jump_map.count_steps(start, end, &directions);

    println!("Steps: {}", steps);
}

enum Direction {
    Left = 0,
    Right = 1,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn parse_directions(lines: &mut Lines<BufReader<File>>) -> Vec<Direction> {
    let line = lines
        .next()
        .expect("Unexpected EOF")
        .expect("Failed to read line");

    line.chars()
        .map(|c| Direction::try_from(c).expect("Unexpected direction char"))
        .collect()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct JumpPosition(u64);

impl TryFrom<&str> for JumpPosition {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(());
        }

        let num_val = u64::from_str_radix(value, 36).map_err(|_| ())?;

        Ok(Self(num_val))
    }
}

struct JumpMap {
    map: HashMap<JumpPosition, (JumpPosition, JumpPosition)>,
}

impl JumpMap {
    pub fn count_steps(
        &self,
        start: JumpPosition,
        end: JumpPosition,
        directions: &[Direction],
    ) -> u64 {
        let mut steps = 0;

        let mut current_pos = start;

        let mut direction_iter = directions.iter().cycle();

        while current_pos != end {
            steps += 1;

            let direction = direction_iter.next().unwrap();

            let target = self.map.get(&current_pos).expect("Invalid jump pos");

            match direction {
                Direction::Left => {
                    current_pos = target.0;
                }
                Direction::Right => {
                    current_pos = target.1;
                }
            }
        }

        steps
    }
}

fn parse_map(lines: &mut Lines<BufReader<File>>) -> JumpMap {
    let mut map = HashMap::new();

    for line in lines.by_ref() {
        let line = line.expect("Failed to read line");

        let (pos_part, jump_part) = line.split_once(" = ").expect("Line split failed");

        let (l_jump, r_jump) = jump_part[1..(jump_part.len() - 1)]
            .split_once(", ")
            .expect("Jump part split failed");

        map.insert(
            JumpPosition::try_from(pos_part).expect("Pos parse failed"),
            (
                JumpPosition::try_from(l_jump).expect("L jump parse failed"),
                JumpPosition::try_from(r_jump).expect("R jump parse failed"),
            ),
        );
    }

    JumpMap { map }
}
