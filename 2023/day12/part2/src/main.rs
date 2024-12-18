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

    let condition_records = parse_condition_records(reader);

    let unfolded_condition_records: Vec<ConditionRecord> = condition_records
        .into_iter()
        .map(|mut cr| {
            let mut conditions_copy = cr.conditions.clone();
            conditions_copy.insert(0, Condition::Unknown);
            let damaged_groups_copy = cr.damaged_groups.clone();

            for _ in 0..4 {
                cr.conditions.append(&mut conditions_copy.clone());
                cr.damaged_groups.append(&mut damaged_groups_copy.clone());
            }

            println!("{:?}", cr);

            cr
        })
        .collect();

    let sum: usize = unfolded_condition_records
        .into_iter()
        .map(|cr| {
            let arr = cr.count_possible_arrangements();

            println!("{arr}");

            arr
        })
        .sum();

    println!("Sum {sum}",);
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Condition {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Condition::Operational),
            '#' => Ok(Condition::Damaged),
            '?' => Ok(Condition::Unknown),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

impl ConditionRecord {
    pub fn count_possible_arrangements(&self) -> usize {
        let mut count = 0;

        for possibility in self.get_all_possible_conditions() {
            let damaged_counts = count_damaged_groups(possibility);

            if damaged_counts == self.damaged_groups {
                count += 1;
            }
        }

        count
    }

    fn get_all_possible_conditions(&self) -> Vec<Vec<Condition>> {
        let mut possibilities = vec![Vec::with_capacity(self.conditions.len())];

        for c in self.conditions.iter() {
            match c {
                Condition::Operational | Condition::Damaged => {
                    possibilities
                        .iter_mut()
                        .for_each(|possibility| possibility.push(*c));
                }
                Condition::Unknown => {
                    possibilities
                        .iter_mut()
                        .for_each(|possibility| possibility.push(Condition::Operational));

                    for i in 0..possibilities.len() {
                        let mut clone = possibilities[i].clone();
                        let len = clone.len();
                        clone[len - 1] = Condition::Damaged;
                        possibilities.push(clone);
                    }
                }
            }
        }

        println!("GOT P");

        possibilities
    }
}

fn count_damaged_groups(conditions: Vec<Condition>) -> Vec<usize> {
    let mut counts = Vec::new();

    let mut in_damaged_group = false;

    for c in conditions {
        match c {
            Condition::Damaged => {
                if !in_damaged_group {
                    counts.push(0);
                    in_damaged_group = true;
                }

                *counts.last_mut().unwrap() += 1;
            }
            Condition::Operational | Condition::Unknown => {
                in_damaged_group = false;
            }
        }
    }

    counts
}

fn parse_condition_records(reader: BufReader<File>) -> Vec<ConditionRecord> {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read line");

            let (conditions_part, groups_part) = line.split_once(' ').expect("Line split failed");

            let conditions = conditions_part
                .chars()
                .map(|c| Condition::try_from(c).expect("Failed to parse condition"))
                .collect();

            let damaged_groups = groups_part
                .split(',')
                .map(|g| g.parse().expect("Group num parse failed"))
                .collect();

            ConditionRecord {
                conditions,
                damaged_groups,
            }
        })
        .collect()
}
