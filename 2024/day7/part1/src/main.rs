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

    let mut equations = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let (result, operands) = line.split_once(": ").expect("Failed to split");

        let operands = operands
            .split(" ")
            .map(|s| s.parse::<isize>().expect("Failed to parse num"))
            .collect::<Vec<_>>();

        equations.push((
            result.parse::<isize>().expect("Failed to parse num"),
            operands,
        ));
    }

    let mut sum = 0;

    for (result, operands) in equations {
        let success = test_equation(result, &operands);

        if success {
            sum += result;
        }
    }

    println!("Sum {sum}");
}

fn test_equation(result: isize, operands: &[isize]) -> bool {
    let mut success = false;

    let num_operators = operands.len() - 1;

    for operator_combination in 0..(2_isize.pow(num_operators as u32)) {
        let res = operands
            .iter()
            .skip(1)
            .enumerate()
            .fold(operands[0], |acc, (idx, op)| {
                if operator_combination & (1 << idx) == 0 {
                    acc + op
                } else {
                    acc * op
                }
            });

        if res == result {
            success = true;
            break;
        }
    }

    success
}
