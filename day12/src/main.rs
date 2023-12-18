//--------------------------------------------------------------------------------
// Day 12: Hot Springs
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::{fmt::Display, fs, collections::BTreeMap};

const FILE_NAME: &'static str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Unknown,
    Functional,
    Broken    
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl From<Condition> for char {
    fn from(value: Condition) -> Self {
        match value {
            Condition::Unknown => '?',
            Condition::Functional => '.',
            Condition::Broken => '#',
        }
    }
}

impl TryFrom<char> for Condition {
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '?' => Ok(Condition::Unknown),
            '.' => Ok(Condition::Functional),
            '#' => Ok(Condition::Broken),
            _ => Err("No Condition equivalent for character.")
        }
    }

    type Error = &'static str;
}

#[derive(Debug)]
struct ConditionRecord {
    condition_symbols: Vec<Condition>,
    group_sizes: Vec<u64>
}

impl ConditionRecord {
    pub fn load_from_file(file_name: &str) -> Vec<ConditionRecord> {
        let mut condition_records: Vec<ConditionRecord> = vec![];
        let Ok(file_contents) = fs::read_to_string(file_name) else {
            return condition_records;
        };

        for line in file_contents.lines() {
            match line.split_whitespace().collect::<Vec<&str>>()[..2] {
                [broken_map_string, group_sizes_string] => {
                    let condition_symbols: Vec<Condition> = Vec::from_iter(
                        broken_map_string.as_bytes()
                        .iter()
                        .map(|byte| Condition::try_from(*byte as char).unwrap_or(Condition::Unknown))
                    );

                    let group_sizes: Vec<u64> = Vec::from_iter(
                        group_sizes_string.split(",")
                        .map(|group_size_string| group_size_string.parse().unwrap())
                    );

                    condition_records.push(
                        ConditionRecord { condition_symbols, group_sizes }
                    )
                }
                _ => {}
            }
        }

        condition_records
    }

    pub fn unfold (&self) -> Self {
        let mut condition_symbols = [&self.condition_symbols[..], &[Condition::Unknown]].concat().repeat(5);
        condition_symbols.pop();
        ConditionRecord {
            condition_symbols: condition_symbols,
            group_sizes: self.group_sizes.repeat(5)
        }
    }

    // pub fn condition_symbols_to_string(&self) -> String {
    //     return String::from_utf8(
    //         self.condition_symbols.iter()
    //                 .map(|condition| char::from(*condition) as u8)
    //                 .collect::<Vec<u8>>()
    //     ).unwrap()
    // }

    fn generate_linear_nfa_from_groups(&self) -> Vec<Condition> {
        // Generate an NFA representing the desired groupings
        // The last two states (# or .) are the accepting states.
        let mut nfa: Vec<Condition> = vec![Condition::Functional];
        for group_size in self.group_sizes.iter() {
            for _ in 0..*group_size {
                nfa.push(Condition::Broken);
            }
            nfa.push(Condition::Functional);
        }

        // println!("{:?}", nfa);
        
        return nfa;
    }


    // The number of arrangements are counted using an NFA.
    // Solution from https://github.com/clrfl/AdventOfCode2023/blob/master/12/part2.py with some modifications
    pub fn count_number_of_arrangements(&self) -> u64 {
        let nfa = self.generate_linear_nfa_from_groups();

        let mut state_frequencies: BTreeMap<u64, u64> = BTreeMap::from([
            (0, 1)
        ]);
        let mut next_state_frequencies: BTreeMap<u64, u64> = BTreeMap::new();

        for input in self.condition_symbols.iter() {
            for kv in &state_frequencies {

                let state_i = kv.0.clone();
                let state: &Condition = nfa.get(state_i as usize).unwrap();
                let state_frequency: u64 = kv.1.clone();

                let next_state_i: u64 = state_i + 1;
                let next_state: Option<&Condition> = nfa.get(next_state_i as usize);
                // let next_state_frequency: u64 = state_frequencies.get(&next_state_i).cloned().unwrap_or(0);

                match input {
                    Condition::Unknown => {
                        if next_state.is_some() {
                            next_state_frequencies.insert(next_state_i, next_state_frequencies.get(&next_state_i).cloned().unwrap_or(0) + state_frequency);
                        }
                        if *state == Condition::Functional {
                            next_state_frequencies.insert(state_i, next_state_frequencies.get(&state_i).cloned().unwrap_or(0) + state_frequency);
                        }
                    },
                    Condition::Functional => {
                        if next_state.is_some() && *next_state.unwrap() == Condition::Functional {
                            next_state_frequencies.insert(next_state_i, next_state_frequencies.get(&next_state_i).cloned().unwrap_or(0) + state_frequency);
                        }
                        if *state == Condition::Functional {
                            next_state_frequencies.insert(state_i, next_state_frequencies.get(&state_i).cloned().unwrap_or(0) + state_frequency);
                        }
                    },
                    Condition::Broken => {
                        if next_state.is_some_and(|next| *next == Condition::Broken) {
                            next_state_frequencies.insert(next_state_i, next_state_frequencies.get(&next_state_i).cloned().unwrap_or(0) + state_frequency);
                        }
                    },
                }
            }
            state_frequencies.clear();
            next_state_frequencies.clone_into(&mut state_frequencies);
            next_state_frequencies.clear();
        }
        // println!("{:?}", &state_frequencies.values().rev().take(2).cloned().sum::<u64>());
        
        // Take the frequencies of the last two states, as they are the only accepting states.
        return state_frequencies.get(&(nfa.len() as u64 - 1)).cloned().unwrap_or(0) + state_frequencies.get(&(nfa.len() as u64 - 2)).cloned().unwrap_or(0)
    }

    pub fn count_all_number_of_arrangements(condition_records: Vec<ConditionRecord>) -> u64 {
        return condition_records
                .iter()
                .map(|condition_record| condition_record.count_number_of_arrangements())
                .sum::<u64>();
    }

    // I initially wanted to use combinations to count, but I eventually realized that there simply were too many cases to solve this with just combinations.
    /* 
    pub fn count_number_of_arrangements_combination(&self) -> u64 {
        let mut number_of_arrangements = 0;

        let condition_symbols_string = self.condition_symbols_to_string();
        let islands = condition_symbols_string.split(".")
            .filter(|island| island.len() > 0).collect::<Vec<&str>>();
        // println!("{:?}", islands.collect::<Vec<&str>>());

        let mut group_sizes: VecDeque<u64> = VecDeque::from_iter(self.group_sizes.iter().cloned());
        let num_of_separators_needed = group_sizes.len() - islands.len();
        /*
        for island in islands {
            let empty_spaces = island.chars().filter(|e| *e == '?').count() as u64;
            let broken = island.chars().count() as u64 - empty_spaces;



            let mut r = 0;
            let mut spaces_needed = 0;

            loop {
                let Some(next_group) = group_sizes.pop_front() else {
                    break;
                };
                if r + next_group + spaces_needed + next_group <= empty_spaces {
                    r += next_group;
                    spaces_needed = r - 1;
                } else {
                    group_sizes.push_front(next_group);
                    break;
                }
            }
            println!("{} taken {}", empty_spaces - spaces_needed, r);
            if r > 0 {
                number_of_arrangements += combination(empty_spaces - spaces_needed, r);
            }
        }
         */

        return number_of_arrangements;
    }
    */
}

// fn combination(n: u64, r: u64) -> u64 {
//     if r == 0 || n == 1 {
//         return 1;
//     }
//     let numerator: u64 = ((n - r + 1)..=n).reduce(|prev, curr| prev * curr).unwrap();   // Cancels numerator and denominator to mitigate overflows
//     let r_factorial: u64 = (1..=r).reduce(|prev, next| prev * next).unwrap();
    
//     return numerator / r_factorial;
// }

fn main() {
    println!("Hello, world!");
    let condition_records: Vec<ConditionRecord> = ConditionRecord::load_from_file(FILE_NAME);
    // for condition_record in condition_records.iter() {
    //     println!("{:?}", condition_record.condition_symbols_to_string());
    //     println!("{:?}", condition_record.count_number_of_arrangements());
    //     println!("{:?}", condition_record.count_number_of_arrangements());
    // }
    let condition_records_unfolded: Vec<ConditionRecord> = condition_records.iter().map(|condition_record| condition_record.unfold()).collect();
    println!("Sum of all counts: {}", ConditionRecord::count_all_number_of_arrangements(condition_records));
    println!("Sum of all counts (unfolded): {}", ConditionRecord::count_all_number_of_arrangements(condition_records_unfolded));
}

#[cfg(test)]
mod test {
    use std::iter::zip;

    use crate::ConditionRecord;

    // #[test]
    // fn test_combination() {
    //     let input_output: Vec<((u64, u64), u64)> = Vec::from([
    //         ((5, 0), 1),
    //         ((4, 4), 1),
    //         ((4, 3), 4),
    //         ((4, 2), 6),
    //         ((4, 1), 4)
    //     ]);

    //     for ((n, r), output) in input_output {
    //         assert_eq!(combination(n, r), output);
    //     }
    // }

    #[test]
    fn test_count_all_number_of_arrangements() {
        let condition_records = ConditionRecord::load_from_file("test_input.txt");
        let correct_answers: Vec<u64> = vec![
            1, 4, 1, 1, 4, 10
        ];

        for (output, correct) in zip(condition_records.iter().map(|condition_record| condition_record.count_number_of_arrangements()), correct_answers) {
            println!("{output} vs {correct}");
            assert_eq!(output, correct);
        }

        let condition_records = ConditionRecord::load_from_file("test_input2.txt");
        let correct_answers: Vec<u64> = vec![
            4
        ];

        for (output, correct) in zip(condition_records.iter().map(|condition_record| condition_record.count_number_of_arrangements()), correct_answers) {
            println!("{output} vs {correct}");
            assert_eq!(output, correct);
        }
    }

    #[test]
    fn test_count_all_number_of_arrangements_unfolded() {
        let condition_records = ConditionRecord::load_from_file("test_input.txt");
        let correct_answers: Vec<u64> = vec![
            1, 16384, 1, 16, 2500, 506250
        ];

        for (output, correct) in zip(condition_records.iter().map(|condition_record| condition_record.unfold().count_number_of_arrangements()), correct_answers) {
            println!("{output} vs {correct}");
            assert_eq!(output, correct);
        }
    }
}