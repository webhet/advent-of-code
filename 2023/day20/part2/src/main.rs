use core::panic;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
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

    let (broadcast_outputs, mut modules) = parse_modules(reader);

    let mut i = 0;

    let button_presses = loop {
        i += 1;

        let rx_pulses = compute_state(&broadcast_outputs, &mut modules);

        if i % 1000 == 0 {
            println!("{rx_pulses} {i}");
        }

        if rx_pulses == 1 {
            break i;
        }
    };

    println!("Button presses: {button_presses}");
}

#[derive(Debug)]
struct Module {
    logic: ModuleLogic,
    outputs: Vec<String>,
}

#[derive(Debug)]
enum ModuleLogic {
    FlipFlop(bool),
    Nand(HashMap<String, bool>),
}

impl ModuleLogic {
    pub fn handle_pulse(&mut self, pulse: bool, from_input: &str) -> Option<bool> {
        match self {
            ModuleLogic::FlipFlop(state) => {
                if pulse {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            ModuleLogic::Nand(state) => {
                let input = state
                    .get_mut(from_input)
                    .expect("Recieved input from unknown source");
                *input = pulse;

                Some(!state.values().all(|v| *v))
            }
        }
    }
}

fn compute_state(broadcast_outputs: &[String], modules: &mut HashMap<String, Module>) -> usize {
    // (target, pulse, source)
    let mut state_deque = VecDeque::from_iter(
        broadcast_outputs
            .iter()
            .map(|name| (name.clone(), false, "broadcaster".to_owned())),
    );

    let mut rx_pulses = 0;

    while let Some((target, pulse, source)) = state_deque.pop_front() {
        if !pulse && target == "rx" {
            rx_pulses += 1;
        }

        let module = modules.get_mut(&target);

        if let Some(module) = module {
            if let Some(next_pulse) = module.logic.handle_pulse(pulse, &source) {
                for o in module.outputs.iter() {
                    state_deque.push_back((o.clone(), next_pulse, target.clone()));
                }
            }
        }
    }

    rx_pulses
}

fn parse_modules(reader: BufReader<File>) -> (Vec<String>, HashMap<String, Module>) {
    let mut modules = HashMap::new();

    let mut broadcast_outputs = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let (name_part, output_part) = line.split_once(" -> ").expect("Line split failed");

        let outputs = output_part.split(", ").map(str::to_owned).collect();

        if name_part == "broadcaster" {
            broadcast_outputs = outputs;
        } else {
            let logic = match &name_part[0..1] {
                "%" => ModuleLogic::FlipFlop(false),
                "&" => ModuleLogic::Nand(HashMap::new()),
                _ => panic!("Unexpected module"),
            };

            modules.insert(
                name_part[1..].to_owned(),
                RefCell::new(Module { logic, outputs }),
            );
        }
    }

    // Find all NAND inputs
    for (module_name, module) in modules.iter() {
        let found_inputs = match module.borrow().logic {
            ModuleLogic::FlipFlop(_) => None,
            ModuleLogic::Nand(_) => Some(
                modules
                    .iter()
                    .filter_map(|(n, m)| {
                        if m.borrow().outputs.contains(module_name) {
                            Some(n.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            ),
        };

        if let Some(found_inputs) = found_inputs {
            let logic = &mut module.borrow_mut().logic;

            if let ModuleLogic::Nand(inputs) = logic {
                for i in found_inputs {
                    inputs.insert(i, false);
                }
            }
        }
    }

    (
        broadcast_outputs,
        modules
            .into_iter()
            .map(|(k, v)| (k, v.into_inner()))
            .collect(),
    )
}
