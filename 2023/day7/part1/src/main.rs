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

    let input_hands = parse_input(reader);

    let mut hands_with_strength: Vec<_> = input_hands
        .into_iter()
        .map(HandWithStrength::from)
        .collect();

    hands_with_strength.sort();

    let sum: u64 = hands_with_strength
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| {
            let rank = (idx + 1) as u64;
            hand.bid * rank
        })
        .sum();

    println!("Sum: {sum}");
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl TryFrom<char> for CardType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(CardType::Two),
            '3' => Ok(CardType::Three),
            '4' => Ok(CardType::Four),
            '5' => Ok(CardType::Five),
            '6' => Ok(CardType::Six),
            '7' => Ok(CardType::Seven),
            '8' => Ok(CardType::Eight),
            '9' => Ok(CardType::Nine),
            'T' => Ok(CardType::Ten),
            'J' => Ok(CardType::Jack),
            'Q' => Ok(CardType::Queen),
            'K' => Ok(CardType::King),
            'A' => Ok(CardType::Ace),
            _ => Err(()),
        }
    }
}

struct InputHand {
    cards: Vec<CardType>,
    bid: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl InputHand {
    pub fn strength(&self) -> HandStrength {
        let mut type_counts: [u8; 13] = [0; 13];
        self.cards
            .iter()
            .for_each(|c_type| type_counts[*c_type as usize] += 1);

        let max = type_counts.iter().max().unwrap();

        match *max {
            5 => HandStrength::FiveOfKind,
            4 => HandStrength::FourOfKind,
            3 => {
                let got_two = type_counts.iter().any(|count| *count == 2);

                if got_two {
                    HandStrength::FullHouse
                } else {
                    HandStrength::ThreeOfKind
                }
            }
            2 => {
                let two_counts =
                    type_counts
                        .iter()
                        .fold(0, |acc, count| if *count == 2 { acc + 1 } else { acc });

                if two_counts == 2 {
                    HandStrength::TwoPair
                } else {
                    HandStrength::OnePair
                }
            }
            _ => HandStrength::HighCard,
        }
    }
}

#[derive(PartialEq, Eq)]
struct HandWithStrength {
    cards: Vec<CardType>,
    bid: u64,
    strength: HandStrength,
}
impl PartialOrd for HandWithStrength {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandWithStrength {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_strength = self.strength as u8;
        let other_strength = other.strength as u8;

        match self_strength.cmp(&other_strength) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                for (idx, card_type) in self.cards.iter().enumerate() {
                    match (*card_type as u8).cmp(&(other.cards[idx] as u8)) {
                        std::cmp::Ordering::Less => {
                            return std::cmp::Ordering::Less;
                        }
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                }

                std::cmp::Ordering::Equal
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

impl From<InputHand> for HandWithStrength {
    fn from(value: InputHand) -> Self {
        let strength = value.strength();

        Self {
            cards: value.cards,
            bid: value.bid,
            strength,
        }
    }
}

fn parse_input(reader: BufReader<File>) -> Vec<InputHand> {
    let mut hands = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let (hand_part, bid_part) = line.split_once(' ').expect("Line split failed");

        let cards = hand_part
            .chars()
            .map(|c| CardType::try_from(c).expect("Invalid card type"))
            .collect();
        let bid = bid_part.parse().expect("Bid parse failed");

        hands.push(InputHand { cards, bid })
    }

    hands
}
