use std::{
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

    let mut game_id_sum = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                println!("Error reading line {err}");
                return;
            }
        };

        let Some((prefix_part, game_part)) = line.split_once(": ") else {
            println!("Got malformed line: {line}");
            return;
        };

        let Ok(game_id) = prefix_part[5..].parse::<u64>() else {
            println!("Malformed game ID: '{}'", &prefix_part[5..]);
            return;
        };

        let game_rounds = match parse_game_part(game_part) {
            Ok(game_rounds) => game_rounds,
            Err(err) => {
                println!("Malformed game: {err}");
                return;
            }
        };

        let all_rounds_possible = game_rounds
            .into_iter()
            .all(|game_round| game_round.is_possible());

        if all_rounds_possible {
            game_id_sum += game_id;
        }
    }

    println!("Game ID sum: {game_id_sum}");
}

struct GameRound {
    red_cubes: u64,
    green_cubes: u64,
    blue_cubes: u64,
}

impl GameRound {
    pub fn is_possible(&self) -> bool {
        const RED_MAX: u64 = 12;
        const GREEN_MAX: u64 = 13;
        const BLUE_MAX: u64 = 14;

        self.red_cubes <= RED_MAX && self.green_cubes <= GREEN_MAX && self.blue_cubes <= BLUE_MAX
    }
}

impl TryFrom<&str> for GameRound {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cube_strs = value.split(", ");

        let mut res = Self {
            red_cubes: 0,
            green_cubes: 0,
            blue_cubes: 0,
        };

        for cube_str in cube_strs {
            let Some((num_str, color_str)) = cube_str.split_once(' ') else {
                return Err(format!("Malformed cube str: {cube_str}"));
            };

            let count = num_str
                .parse::<u64>()
                .map_err(|err| format!("Malformed cobe count: {num_str}; {err}"))?;

            match color_str {
                "red" => res.red_cubes += count,
                "green" => res.green_cubes += count,
                "blue" => res.blue_cubes += count,
                _ => {
                    return Err(format!("Unknown color: {color_str}"));
                }
            }
        }

        Ok(res)
    }
}

fn parse_game_part(game_part: &str) -> Result<Vec<GameRound>, String> {
    let game_round_strs = game_part.split("; ");

    let mut game_rounds = Vec::new();

    for game_round_str in game_round_strs {
        game_rounds.push(GameRound::try_from(game_round_str)?);
    }

    Ok(game_rounds)
}
