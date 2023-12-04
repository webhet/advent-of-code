use std::{
    collections::{HashSet, VecDeque},
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

    let mut card_count = 0;

    let mut copy_deque = VecDeque::new();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                println!("Error reading line {err}");
                return;
            }
        };

        let current_card_copies = copy_deque.pop_front().unwrap_or(0);
        // Copies + the card itself.
        let current_card_count = current_card_copies + 1;

        card_count += current_card_count;

        let card = parse_card_from_line(&line);
        let wins = card.number_of_wins();

        for i in 0..wins {
            match copy_deque.get_mut(i) {
                Some(e) => {
                    *e += current_card_count;
                }
                None => {
                    copy_deque.push_back(current_card_count);
                }
            }
        }
    }

    println!("Card count: {card_count}");
}

#[derive(Debug)]
struct Card {
    _num: u64,
    winning_nums: HashSet<u64>,
    your_nums: HashSet<u64>,
}

impl Card {
    pub fn number_of_wins(&self) -> usize {
        self.winning_nums.intersection(&self.your_nums).count()
    }
}

fn parse_card_from_line(line: &str) -> Card {
    let (card_part, nums_part) = line.split_once(": ").expect("Card split failed");
    let num = card_part
        .split_once(' ')
        .expect("Card num split failed")
        .1
        .trim()
        .parse()
        .expect("Card num parse failed");

    let (lnums, rnums) = nums_part.split_once(" | ").expect("Nums split failed");

    Card {
        _num: num,
        winning_nums: parse_set_from_line(lnums),
        your_nums: parse_set_from_line(rnums),
    }
}

fn parse_set_from_line(line: &str) -> HashSet<u64> {
    let num_strs = line.split(' ');

    let mut set = HashSet::new();

    for num_str in num_strs {
        if !num_str.is_empty() {
            let num = num_str.parse().expect("Num parsing pailed");
            set.insert(num);
        }
    }

    set
}
