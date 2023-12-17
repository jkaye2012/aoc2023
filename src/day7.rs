fn face_score(ch: char, use_wildcard: bool) -> u8 {
    if use_wildcard {
        match ch {
            'J' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("Invalid face"),
        }
    } else {
        match ch {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("Invalid face"),
        }
    }
}

#[repr(u8)]
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeKind = 3,
    FullHouse = 4,
    FourKind = 5,
    FiveKind = 6,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Hand {
    htype: HandType,
    orig: [u8; 5],
    bid: usize,
}

fn hand_type(counts: &[u8]) -> HandType {
    let mut htype = HandType::HighCard;
    for &c in counts {
        if c == 5 {
            htype = HandType::FiveKind;
            break;
        } else if c == 4 {
            htype = HandType::FourKind;
            break;
        } else if c == 3 {
            if htype == HandType::OnePair {
                htype = HandType::FullHouse;
                break;
            } else {
                htype = HandType::ThreeKind;
            }
        } else if c == 2 {
            if htype == HandType::ThreeKind {
                htype = HandType::FullHouse;
                break;
            } else if htype == HandType::OnePair {
                htype = HandType::TwoPair;
                break;
            } else {
                htype = HandType::OnePair;
            }
        }
    }
    htype
}

fn apply_wildcard(htype: HandType, joker_count: u8) -> HandType {
    if joker_count == 5 || joker_count == 4 {
        HandType::FiveKind
    } else if joker_count == 3 {
        if htype == HandType::OnePair {
            HandType::FiveKind
        } else {
            HandType::FourKind
        }
    } else if joker_count == 2 {
        if htype == HandType::ThreeKind {
            HandType::FiveKind
        } else if htype == HandType::OnePair {
            HandType::FourKind
        } else {
            HandType::ThreeKind
        }
    } else if joker_count == 1 {
        if htype == HandType::FourKind {
            HandType::FiveKind
        } else if htype == HandType::ThreeKind {
            HandType::FourKind
        } else if htype == HandType::TwoPair {
            HandType::FullHouse
        } else if htype == HandType::OnePair {
            HandType::ThreeKind
        } else {
            HandType::OnePair
        }
    } else {
        htype
    }
}

impl Hand {
    pub fn new(orig: [u8; 5], bid: usize, use_wildcard: bool) -> Hand {
        if use_wildcard {
            Self::with_wildcard(orig, bid)
        } else {
            let mut counts = [0u8; 13];
            for &c in orig.iter() {
                counts[c as usize] += 1;
            }
            let htype = hand_type(&counts);
            Hand { htype, orig, bid }
        }
    }

    pub fn with_wildcard(orig: [u8; 5], bid: usize) -> Hand {
        let mut counts = [0; 13];
        for c in orig {
            counts[c as usize] += 1;
        }
        let mut htype = hand_type(&counts[1..]);
        htype = apply_wildcard(htype, counts[0]);

        Hand { htype, orig, bid }
    }
}

fn generator(input: &str, use_wildcard: bool) -> Vec<Hand> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|i| {
            let (raw_hand, bid) = i.split_once(' ').unwrap();
            let mut hand = [0u8; 5];
            raw_hand
                .chars()
                .enumerate()
                .for_each(|(i, c)| hand[i] = face_score(c, use_wildcard));
            Hand::new(hand, bid.parse().unwrap(), use_wildcard)
        })
        .collect();
    hands.sort();
    hands
}

#[aoc_generator(day7, part1)]
pub fn generate(input: &str) -> Vec<Hand> {
    generator(input, false)
}

#[aoc_generator(day7, part2)]
pub fn generate_wildcard(input: &str) -> Vec<Hand> {
    generator(input, true)
}

#[aoc(day7, part1)]
#[aoc(day7, part2)]
pub fn total_winnings(hands: &[Hand]) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum()
}
