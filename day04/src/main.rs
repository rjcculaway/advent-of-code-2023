
//--------------------------------------------------------------------------------
// Day 03: Scratchcards
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::collections::HashSet;
use std::fs;

const FILE_NAME: &str = "input.txt";

#[derive(Debug)]
#[derive(PartialEq)]
struct Scratchcard {
    card_number: u8,
    winning_numbers: HashSet<u64>,
    owned_numbers: HashSet<u64>,
    quantity: u32
}

impl Scratchcard {
    pub fn count_winning_matches(&self) -> usize {
        return self.winning_numbers.intersection(&self.owned_numbers).collect::<Vec<&u64>>().len()
    }
    pub fn compute_card_value(&self) -> u64 {
        let num_of_winning_matches = self.count_winning_matches();
        if num_of_winning_matches < 1 {
            return 0;
        }
        return 2_u64.pow(num_of_winning_matches.try_into().unwrap_or(0) - 1);
    }
}

fn obtain_scratchcard_copies(pile: &mut Vec<Scratchcard>) {
    for i in 0..pile.len() {
        let num_of_winning_matches = pile[i].count_winning_matches();
        for j in i + 1..=i + num_of_winning_matches {
            pile[j].quantity += pile[i].quantity;
        }
    }
}

fn load_scratchcards(file_name: &str) -> Vec<Scratchcard> {
    let mut scratchcards: Vec<Scratchcard> = Vec::new();

    if let Ok(file_contents) = fs::read_to_string(file_name) {
        for line in file_contents.lines() {
            let card_and_card_contents: Vec<&str> = line.split(": ").collect();
            let card_number: u8 = card_and_card_contents[0].split_whitespace().nth(1).unwrap().parse().unwrap();

            let card_sections: Vec<&str> = card_and_card_contents[1].split(" | ").collect();

            let mut winning_numbers: HashSet<u64> = HashSet::new();
            let mut owned_numbers: HashSet<u64> = HashSet::new();

            for number in card_sections[0].split_whitespace() {
                winning_numbers.insert(number.parse::<u64>().unwrap());
            }
            for number in card_sections[1].split_whitespace() {
                owned_numbers.insert(number.parse::<u64>().unwrap());
            }
                
            scratchcards.push(Scratchcard { card_number: card_number, winning_numbers: winning_numbers, owned_numbers: owned_numbers, quantity: 1 });
        }
    }
    obtain_scratchcard_copies(&mut scratchcards);
    return scratchcards;
}

fn compute_scratchcard_pile(pile: &Vec<Scratchcard>) -> u64 {
    return pile.iter().map(|scratchcard: &Scratchcard| return scratchcard.compute_card_value()).reduce(|prev, curr| return prev + curr).unwrap();
}

fn count_total_scratchcards(pile: &Vec<Scratchcard>) -> u32 {
    return pile.iter().map(|scratchcard: &Scratchcard| return scratchcard.quantity).reduce(|prev: u32, next: u32| return prev + next).unwrap_or(0);
}

fn main() {
    let scratchcards: Vec<Scratchcard> = load_scratchcards(FILE_NAME);
    println!("Scratchcard pile value: {}", compute_scratchcard_pile(&scratchcards));
    println!("Total scratchcards: {}", count_total_scratchcards(&scratchcards));
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{load_scratchcards, Scratchcard, compute_scratchcard_pile, count_total_scratchcards};

    #[test]
    fn test_load_scratchcards() {
        assert_eq!(load_scratchcards("test_input.txt"), vec![
            Scratchcard { 
                card_number: 1,
                winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
                owned_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
                quantity: 1
             },
            Scratchcard { 
                card_number: 2,
                winning_numbers: HashSet::from([13, 32, 20, 16, 61]),
                owned_numbers: HashSet::from([61, 30, 68, 82, 17, 32, 24, 19]),
                quantity: 2
             },
            Scratchcard{
                card_number: 3,
                winning_numbers: HashSet::from([1, 21, 53, 59, 44]),
                owned_numbers: HashSet::from([69, 82, 63, 72, 16, 21, 14,  1]),
                quantity: 4
            },
            Scratchcard{
                card_number: 4,
                winning_numbers: HashSet::from([41, 92, 73, 84, 69]),
                owned_numbers: HashSet::from([59, 84, 76, 51, 58, 5, 54, 83]),
                quantity: 8
            },
            Scratchcard{
                card_number: 5,
                winning_numbers: HashSet::from([87, 83, 26, 28, 32]),
                owned_numbers: HashSet::from([88, 30, 70, 12, 93, 22, 82, 36]),
                quantity: 14
            },
            Scratchcard{
                card_number: 6, winning_numbers: HashSet::from([31, 18, 13, 56, 72]),
                owned_numbers: HashSet::from([74, 77, 10, 23, 35, 67, 36, 11]),
                quantity: 1
            },
        ]);
    }
    #[test]
    fn test_compute_card_value() {
        let scratchcards: Vec<Scratchcard> = load_scratchcards("test_input.txt");
        let correct_card_values: [u64; 6] = [8, 2, 2, 1, 0, 0];

        for i in 0..6 {
            assert_eq!(scratchcards[i].compute_card_value(), correct_card_values[i]);
        }
    }

    #[test]
    fn test_card_pile_total() {
        assert_eq!(compute_scratchcard_pile(&load_scratchcards("test_input.txt")), 13);
    }

    #[test]
    fn test_count_total_scratchcards() {
        assert_eq!(count_total_scratchcards(&load_scratchcards("test_input.txt")), 30);
    }
}