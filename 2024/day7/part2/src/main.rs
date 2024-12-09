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
        let success = test_equation(result, operands[0], &operands[1..]);

        if success {
            sum += result;
        }
    }

    println!("Sum {sum}");
}

fn test_equation(result: isize, acc: isize, operands: &[isize]) -> bool {
    if operands.is_empty() {
        return result == acc;
    }

    // *
    let mut new_acc = acc * operands[0];

    if test_equation(result, new_acc, &operands[1..]) {
        return true;
    }

    // +
    new_acc = acc + operands[0];

    if test_equation(result, new_acc, &operands[1..]) {
        return true;
    }

    // ||
    new_acc = format!("{}{}", acc, operands[0])
        .parse()
        .expect("Failed to parse num");

    test_equation(result, new_acc, &operands[1..])
}
