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

    let workflow_map = parse_workflows(&mut lines);
    let parts = parse_parts(&mut lines);

    let sum: usize = parts
        .iter()
        .map(|part| {
            let starting_workflow = workflow_map.get("in").expect("No 'in' workflow found");

            if part.apply_workflow(starting_workflow, &workflow_map) {
                part.xmas_sum()
            } else {
                0
            }
        })
        .sum();

    println!("Sum {sum}");
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Debug)]
enum Rule {
    Conditional {
        lcond: PartValue,
        op: Op,
        rcond: usize,
        result: RuleResult,
    },
    Unconditional(RuleResult),
}

impl Rule {
    pub fn eval_for_part(&self, part: &Part) -> Option<&RuleResult> {
        match self {
            Rule::Conditional {
                lcond,
                op,
                rcond,
                result,
            } => {
                let lcond = part.value(*lcond);
                let matches = match op {
                    Op::Greater => lcond > *rcond,
                    Op::Less => lcond < *rcond,
                };

                if matches {
                    Some(result)
                } else {
                    None
                }
            }
            Rule::Unconditional(res) => Some(res),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PartValue {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Op {
    Greater,
    Less,
}

#[derive(Debug)]
enum RuleResult {
    Accepted,
    Rejected,
    OtherRule(String),
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    pub fn apply_workflow(
        &self,
        workflow: &Workflow,
        workflow_map: &HashMap<String, Workflow>,
    ) -> bool {
        for rule in workflow.rules.iter() {
            if let Some(res) = rule.eval_for_part(self) {
                match res {
                    RuleResult::Accepted => {
                        return true;
                    }
                    RuleResult::Rejected => {
                        return false;
                    }
                    RuleResult::OtherRule(rule_name) => {
                        let other_workflow = workflow_map
                            .get(rule_name)
                            .expect("Other rule doesn't exist");

                        return self.apply_workflow(other_workflow, workflow_map);
                    }
                }
            }
        }

        unreachable!()
    }

    pub fn value(&self, part_value: PartValue) -> usize {
        match part_value {
            PartValue::X => self.x,
            PartValue::M => self.m,
            PartValue::A => self.a,
            PartValue::S => self.s,
        }
    }

    pub fn xmas_sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse_workflows(lines: &mut Lines<BufReader<File>>) -> HashMap<String, Workflow> {
    let mut map = HashMap::new();

    loop {
        let line = lines
            .next()
            .expect("Unexpected EOF")
            .expect("Failed to read line");

        if line.is_empty() {
            break;
        }

        let (rule_name, rule) = line.split_once('{').expect("Rule split failed");
        let rule_parts = rule[0..rule.len() - 1].split(',');

        let mut rules = Vec::new();

        for rule_part in rule_parts {
            if rule_part.contains(':') {
                let (cond_part, res) = rule_part.split_once(':').unwrap();

                let (lcond, op, rcond) = if cond_part.contains('<') {
                    let (lcond, rcond) = cond_part.split_once('<').unwrap();
                    (lcond, Op::Less, rcond)
                } else {
                    let (lcond, rcond) = cond_part.split_once('>').expect("Greater split failed");
                    (lcond, Op::Greater, rcond)
                };

                let result = match res {
                    "A" => RuleResult::Accepted,
                    "R" => RuleResult::Rejected,
                    p => RuleResult::OtherRule(p.to_owned()),
                };

                let lcond = match lcond {
                    "x" => PartValue::X,
                    "m" => PartValue::M,
                    "a" => PartValue::A,
                    "s" => PartValue::S,
                    _ => panic!("Unexpected part value"),
                };

                let rcond = rcond.parse().expect("Num parse failed");

                rules.push(Rule::Conditional {
                    lcond,
                    op,
                    rcond,
                    result,
                });
            } else {
                rules.push(Rule::Unconditional(match rule_part {
                    "A" => RuleResult::Accepted,
                    "R" => RuleResult::Rejected,
                    p => RuleResult::OtherRule(p.to_owned()),
                }))
            }
        }

        map.insert(rule_name.to_owned(), Workflow { rules });
    }

    map
}

fn parse_parts(lines: &mut Lines<BufReader<File>>) -> Vec<Part> {
    lines
        .map(|line| {
            let line = line.expect("Failed to read line");
            let num_str = line
                .replace("{x=", "")
                .replace("m=", "")
                .replace("a=", "")
                .replace("s=", "")
                .replace('}', "");

            let nums = num_str
                .split(',')
                .map(|str| str.parse().expect("Num parse failed"))
                .collect::<Vec<_>>();

            Part {
                x: nums[0],
                m: nums[1],
                a: nums[2],
                s: nums[3],
            }
        })
        .collect()
}
