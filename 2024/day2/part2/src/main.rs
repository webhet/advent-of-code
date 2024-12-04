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

    let mut reports = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let report = line
            .split(" ")
            .into_iter()
            .map(|num| num.parse::<isize>().expect("Num parse failed"))
            .collect::<Vec<_>>();

        reports.push(report);
    }

    let mut safe_report_count = 0;

    'outer: for report in reports {
        if is_report_safe(&report) {
            safe_report_count += 1;
            continue 'outer;
        }

        for idx in 0..report.len() {
            let modified_report = report
                .iter()
                .enumerate()
                .filter_map(|(i, val)| if idx == i { None } else { Some(*val) })
                .collect::<Vec<_>>();

            if is_report_safe(&modified_report) {
                safe_report_count += 1;
                continue 'outer;
            }
        }
    }

    println!("Safe: {safe_report_count}");
}

fn is_report_safe(report: &[isize]) -> bool {
    let diffs = report.windows(2).into_iter().map(|win| win[0] - win[1]);

    let mut sign = None;
    let mut safe = true;

    for diff in diffs {
        if diff == 0 || diff.abs() > 3 {
            safe = false;
            break;
        } else if sign.is_some() && sign != Some(diff.signum()) {
            safe = false;
            break;
        }

        sign = Some(diff.signum());
    }

    safe
}
