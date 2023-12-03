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

    let mut sum_of_set_powers = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                println!("Error reading line {err}");
                return;
            }
        };

        let Some((_, game_part)) = line.split_once(": ") else {
            println!("Got malformed line: {line}");
            return;
        };

        let game_rounds = match parse_game_part(game_part) {
            Ok(game_rounds) => game_rounds,
            Err(err) => {
                println!("Malformed game: {err}");
                return;
            }
        };

        let min_set_power = get_min_set(&game_rounds).get_set_power();

        sum_of_set_powers += min_set_power;
    }

    println!("Min set power sum: {sum_of_set_powers}");
}

fn get_min_set(game_rounds: &[GameRound]) -> GameRound {
    fn max(a: Option<u64>, b: Option<u64>) -> Option<u64> {
        match (a, b) {
            (None, None) => None,
            (None, Some(b)) => Some(b),
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(std::cmp::max(a, b)),
        }
    }

    let (r_min, g_min, b_min) = game_rounds
        .iter()
        .fold((None, None, None), |acc, game_round| {
            (
                max(acc.0, game_round.red_cubes),
                max(acc.1, game_round.green_cubes),
                max(acc.2, game_round.blue_cubes),
            )
        });

    GameRound {
        red_cubes: r_min,
        green_cubes: g_min,
        blue_cubes: b_min,
    }
}

struct GameRound {
    red_cubes: Option<u64>,
    green_cubes: Option<u64>,
    blue_cubes: Option<u64>,
}

impl GameRound {
    pub fn get_set_power(&self) -> u64 {
        self.red_cubes.unwrap_or(1) * self.green_cubes.unwrap_or(1) * self.blue_cubes.unwrap_or(1)
    }
}

impl TryFrom<&str> for GameRound {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cube_strs = value.split(", ");

        let mut res = Self {
            red_cubes: None,
            green_cubes: None,
            blue_cubes: None,
        };

        for cube_str in cube_strs {
            let Some((num_str, color_str)) = cube_str.split_once(' ') else {
                return Err(format!("Malformed cube str: {cube_str}"));
            };

            let count = num_str
                .parse::<u64>()
                .map_err(|err| format!("Malformed cobe count: {num_str}; {err}"))?;

            match color_str {
                "red" => res.red_cubes = Some(count),
                "green" => res.green_cubes = Some(count),
                "blue" => res.blue_cubes = Some(count),
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
