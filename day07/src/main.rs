//--------------------------------------------------------------------------------
// Day 07: Camel Cards
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::collections::HashMap;
use std::fs;
use std::cmp::Ordering;
use std::iter::zip;

// #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// enum CardOld {
//     TWO, 
//     THREE, 
//     FOUR, 
//     FIVE, 
//     SIX, 
//     SEVEN, 
//     EIGHT, 
//     NINE, 
//     T, 
//     J,
//     Q, 
//     K, 
//     A
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    J,
    TWO, 
    THREE, 
    FOUR, 
    FIVE, 
    SIX, 
    SEVEN, 
    EIGHT, 
    NINE, 
    T, 
    Q, 
    K, 
    A
}

impl From<Card> for u8 {
    fn from(value: Card) -> Self {
        return value as u8;
    }
}

impl From<Card> for char {
    fn from(value: Card) -> Self {
        return value as u8 as char;
    }
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            b'2' => Card::TWO, 
            b'3' => Card::THREE, 
            b'4' => Card::FOUR, 
            b'5' => Card::FIVE, 
            b'6' => Card::SIX, 
            b'7' => Card::SEVEN, 
            b'8' => Card::EIGHT, 
            b'9' => Card::NINE, 
            b'T' => Card::T, 
            b'J' => Card::J, 
            b'Q' => Card::Q, 
            b'K' => Card::K, 
            b'A' => Card::A,
            _ => Card::TWO,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        return (value as u8).into();
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u16
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cards == other.cards;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.cards != other.cards;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_type_comparison = self.get_hand_type().cmp(&other.get_hand_type());
        // println!("{:#?} vs. {:#?}", self.get_hand_type(), &other.get_hand_type());
        match hand_type_comparison {
            Ordering::Equal => {    // Compare each card in both hands
                for (card_self, card_other) in zip(self.cards, other.cards) {
                    match card_self.cmp(&card_other) {
                        Ordering::Equal => {},
                        card_ordering @ _ => {
                            // println!("{:#?} vs. {:#?} = {:#?}", card_self, card_other, card_ordering);
                            return Some(card_ordering);
                        }
                    }
                }
                return Some(Ordering::Equal);
            },
            _ => {  // Just use hand type for comparison
                return Some(hand_type_comparison);
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl Hand {
    pub fn get_hand_type(&self) -> HandType {
        let mut card_frequencies: HashMap<Card, u8> = HashMap::new();
        let mut num_of_jacks: u8 = 0;

        for &card in &self.cards {
            if card != Card::J {
                match card_frequencies.get(&card) {
                    Some(frequency) => {
                        card_frequencies.insert(card, frequency + 1);
                    },
                    None => {
                        card_frequencies.insert(card, 1);
                    }
                }
            } else {
                num_of_jacks += 1;
            }
        }

        let mut frequencies_only: Vec<u8> = card_frequencies.values().map(|val| *val).collect::<Vec<u8>>();
        frequencies_only.sort();
        if let Some(last) = frequencies_only.last_mut() {
            *last = *last + num_of_jacks;
        }

        // println!("{:#?}", frequencies_only);

        match frequencies_only[..] {
            [5] | [] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard
        }
    }

    pub fn get_total_winnings(hands: &Vec<Hand>) -> u64 {
        return hands.iter()
                    .map(|hand| return hand.bid as u64)
                    .enumerate()
                    .map(|(rank, bid)| return (rank as u64 + 1) * bid)
                    .reduce(|prev, curr| return prev + curr)
                    .unwrap();
    }

    pub fn load_from_file(file_name: &str) -> Vec<Hand> {
        let mut hands: Vec<Hand> = vec![];
        if let Ok(file_contents) = fs::read_to_string(file_name) {
            for line in file_contents.lines() {
                match line.split_whitespace().collect::<Vec<&str>>()[..2] {
                    [cards_part, bid_part] => {
                        hands.push(
                            Hand { 
                                cards: cards_part.as_bytes().iter().map(|byte| Card::from(*byte)).collect::<Vec<Card>>().try_into().unwrap(), 
                                bid: bid_part.parse().unwrap_or(0) 
                            }
                        );
                    },
                    _ => {}
                }
            }
        }
        return hands;
    }
}

const FILE_NAME: &'static str = "input.txt";

fn main() {
    let mut hands = Hand::load_from_file(FILE_NAME);
    hands.sort();
    println!("Total Winnings: {}", Hand::get_total_winnings(&hands));
}

#[cfg(test)]
mod tests {
    use crate::Hand;

    #[test]
    fn test_load_from_file() {
        assert_eq!(Hand::load_from_file("test_input.txt"), vec![
            Hand { cards: ['3'.into(), '2'.into(), 'T'.into(), '3'.into(), 'K'.into()], bid: 765 },
            Hand { cards: ['T'.into(), '5'.into(), '5'.into(), 'J'.into(), '5'.into()], bid: 684 },
            Hand { cards: ['K'.into(), 'K'.into(), '6'.into(), '7'.into(), '7'.into()], bid: 28 },
            Hand { cards: ['K'.into(), 'T'.into(), 'J'.into(), 'J'.into(), 'T'.into()], bid: 220 },
            Hand { cards: ['Q'.into(), 'Q'.into(), 'Q'.into(), 'J'.into(), 'A'.into()], bid: 483 },
        ])
    }

    // Will only work if you use CardOld
    // #[test]
    // fn test_hand_ranking() {
    //     let mut hands = Hand::load_from_file("test_input.txt");
    //     hands.sort();
    //     assert_eq!(hands, vec![
    //         Hand { cards: ['3'.into(), '2'.into(), 'T'.into(), '3'.into(), 'K'.into()], bid: 765 },
    //         Hand { cards: ['K'.into(), 'T'.into(), 'J'.into(), 'J'.into(), 'T'.into()], bid: 220 },
    //         Hand { cards: ['K'.into(), 'K'.into(), '6'.into(), '7'.into(), '7'.into()], bid: 28 },
    //         Hand { cards: ['T'.into(), '5'.into(), '5'.into(), 'J'.into(), '5'.into()], bid: 684 },
    //         Hand { cards: ['Q'.into(), 'Q'.into(), 'Q'.into(), 'J'.into(), 'A'.into()], bid: 483 },
    //     ])
    // }

    #[test]
    fn get_total_winnings() {
        let mut hands = Hand::load_from_file("test_input.txt");
        hands.sort();
        assert_eq!(Hand::get_total_winnings(&hands), 5905);
    }

}

