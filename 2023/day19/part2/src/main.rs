use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Lines},
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

    let mut lines = reader.lines();

    let workflow_map = parse_workflows(&mut lines);

    let start_ranges = XmasRangeSet {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };

    let start_workflow = workflow_map.get("in").expect("No 'in'workflow found");

    let sum = apply_range_set_for_workflow(start_ranges, start_workflow, &workflow_map);

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

#[derive(Debug, Clone, Copy)]
enum PartValue {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
struct XmasRangeSet {
    x: std::ops::Range<usize>,
    m: std::ops::Range<usize>,
    a: std::ops::Range<usize>,
    s: std::ops::Range<usize>,
}

impl XmasRangeSet {
    pub fn possible_combinations(&self) -> usize {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }

    pub fn split_matching_not_matching(
        self,
        lcond: PartValue,
        op: Op,
        rcond: usize,
    ) -> (Self, Self) {
        let range = match lcond {
            PartValue::X => &self.x,
            PartValue::M => &self.m,
            PartValue::A => &self.a,
            PartValue::S => &self.s,
        };

        let (matching, not_matching) = match op {
            Op::Greater => {
                let matching_start = std::cmp::max(range.start, rcond + 1);
                let matching_end = std::cmp::max(range.end, matching_start);

                let not_matching_start = std::cmp::min(range.start, rcond + 1);
                let not_matching_end = std::cmp::min(range.end, rcond + 1);

                (
                    matching_start..matching_end,
                    not_matching_start..not_matching_end,
                )
            }
            Op::Less => {
                let matching_end = std::cmp::min(range.end, rcond);
                let matching_start = std::cmp::min(range.start, matching_end);

                let not_matching_end = std::cmp::max(range.end, rcond);
                let not_matching_start = std::cmp::max(range.start, rcond);

                (
                    matching_start..matching_end,
                    not_matching_start..not_matching_end,
                )
            }
        };

        match lcond {
            PartValue::X => (
                Self {
                    x: matching,
                    m: self.m.clone(),
                    a: self.a.clone(),
                    s: self.s.clone(),
                },
                Self {
                    x: not_matching,
                    m: self.m.clone(),
                    a: self.a.clone(),
                    s: self.s.clone(),
                },
            ),
            PartValue::M => (
                Self {
                    x: self.x.clone(),
                    m: matching,
                    a: self.a.clone(),
                    s: self.s.clone(),
                },
                Self {
                    x: self.x.clone(),
                    m: not_matching,
                    a: self.a.clone(),
                    s: self.s.clone(),
                },
            ),
            PartValue::A => (
                Self {
                    x: self.x.clone(),
                    m: self.m.clone(),
                    a: matching,
                    s: self.s.clone(),
                },
                Self {
                    x: self.x.clone(),
                    m: self.m.clone(),
                    a: not_matching,
                    s: self.s.clone(),
                },
            ),
            PartValue::S => (
                Self {
                    x: self.x.clone(),
                    m: self.m.clone(),
                    a: self.a.clone(),
                    s: matching,
                },
                Self {
                    x: self.x.clone(),
                    m: self.m.clone(),
                    a: self.a.clone(),
                    s: not_matching,
                },
            ),
        }
    }
}

fn apply_range_set_for_workflow(
    start_range_set: XmasRangeSet,
    workflow: &Workflow,
    workflow_map: &HashMap<String, Workflow>,
) -> usize {
    let mut sum = 0;

    let mut remaining_range_set = start_range_set;

    for rule in workflow.rules.iter() {
        match rule {
            Rule::Conditional {
                lcond,
                op,
                rcond,
                result,
            } => {
                let (matching, not_matching) = remaining_range_set
                    .clone()
                    .split_matching_not_matching(*lcond, *op, *rcond);

                sum += match result {
                    RuleResult::Accepted => matching.possible_combinations(),
                    RuleResult::Rejected => 0,
                    RuleResult::OtherRule(workflow_name) => {
                        let next_workflow = workflow_map
                            .get(workflow_name)
                            .expect("No workflow with found for given name");
                        apply_range_set_for_workflow(matching.clone(), next_workflow, workflow_map)
                    }
                };

                remaining_range_set = not_matching;
            }
            Rule::Unconditional(res) => {
                sum += match res {
                    RuleResult::Accepted => remaining_range_set.possible_combinations(),
                    RuleResult::Rejected => 0,
                    RuleResult::OtherRule(workflow_name) => {
                        let next_workflow = workflow_map
                            .get(workflow_name)
                            .expect("No workflow with found for given name");
                        apply_range_set_for_workflow(
                            remaining_range_set.clone(),
                            next_workflow,
                            workflow_map,
                        )
                    }
                };
            }
        }
    }

    sum
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
