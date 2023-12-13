//--------------------------------------------------------------------------------
// Day 08: Haunted Wasteland
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::{fs, collections::HashMap, iter::zip};

const FILE_NAME: &str = "input.txt";

#[derive(Debug)]
enum MovementInstruction {
    LEFT,
    RIGHT
}

impl TryFrom<char> for MovementInstruction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(MovementInstruction::LEFT),
            'R' => Ok(MovementInstruction::RIGHT),
            _ => Err("Failed to convert character into MovementInstruction.")
        }
    }
}

fn gcd(a: u128, b: u128) -> Option<u128> {
    let mut numbers = (a, b);
    loop {
        match numbers {
            (0, 0) => { return None },
            (divisor @ _, 0) | (0, divisor @ _) => { return Some(divisor); },
            (left @ _, right @ _) => {
                numbers.0 = right;
                numbers.1 = left % right;
                continue;
            }
        }
    }
}

fn lcm(a: u128, b: u128) -> Option<u128> {
    match gcd(a, b) {
        None => None,
        Some(gcd_of_values) => {
            Some(a * b / gcd_of_values)
        }

    }
}

// fn gcd_multiple<'a, I>(values: I) -> Option<u128> where I: IntoIterator<Item=u128> {
//     let mut values_iterator = values.into_iter();
//     let mut result: Option<u128> = values_iterator.next();
//     for value in values_iterator {
//         match result {
//             None => { return None; },
//             Some(current_gcd) => {
//                 result = gcd(current_gcd, value);
//             }
//         }
//     }
//     return result;
// }

fn lcm_multiple<'a, I>(values: I) -> Option<u128> where I: IntoIterator<Item=u128> {
    let mut values_iter = values.into_iter();
    let mut current_lcm = values_iter.next();
    for value in values_iter {
        match current_lcm {
            None => { return None; },
            Some(valid_lcm) => {
                current_lcm = lcm(valid_lcm, value);
            }
        }
    }
    return current_lcm;
}

#[derive(Debug)]
struct Map {
    graph: HashMap<String, [String; 2]>,
    movement_instructions: Vec<MovementInstruction>,
    root: String
}

impl Map {
    pub fn traverse_map(&self) -> u128 {
        let mut current_node: &String = &self.root;
        let mut steps_traversed: u128 = 0;
        for movement_direction in self.movement_instructions.iter().cycle() {
            if current_node == "ZZZ" { break; }
            match self.graph.get(current_node) {
                Some([left, right]) => {
                    let next_node = match movement_direction {
                        MovementInstruction::LEFT => left,
                        MovementInstruction::RIGHT => right
                    };

                    if next_node == current_node {
                        return 0;
                    } else {
                        current_node = next_node;
                        steps_traversed += 1;
                    }
                },
                None => { return 0; }
            }
        }

        return steps_traversed;
    }

    fn is_starting_node(key: &String) -> bool {
        return key.chars().last().unwrap() == 'A';
    }

    fn is_ending_node(key: &String) -> bool {
        return key.chars().last().unwrap() == 'Z';
    }

    fn get_starting_nodes(&self) -> Vec<String> {
        return self.graph.keys().filter(|key| Map::is_starting_node(key)).map(|key| key.clone()).collect();
    }

    fn get_next(&self, current_node: &String, direction: &MovementInstruction) -> Option<String> {
        
        if let Some([left, right]) = self.graph.get(current_node) {
            return match direction {
                MovementInstruction::LEFT => Some(left.to_string()),
                MovementInstruction::RIGHT => Some(right.to_string())
            }
        }
        
        return None;
    }

    // Did a bit Googling for this one, and it turns out that getting the LCM is the quick solution for this one.
    // It makes sense, especially finding out we will get to a point that a path will loop in of itself. I thought the Map was DAG!
    // The problem itself was a bit confusing.
    // https://www.reddit.com/r/adventofcode/comments/18df7px/comment/kcxknma/?utm_source=share&utm_medium=web2x&context=3
    pub fn traverse_map_as_ghost(&self) -> u128 {
        let mut steps: u128 = 0;

        let mut cycle_repetition: HashMap<String, u128> = HashMap::new();
        let starting_nodes: Vec<String> = self.get_starting_nodes();
        let mut current_nodes: Vec<String> = self.get_starting_nodes();

        let num_of_starting_nodes: usize = starting_nodes.len();

        for direction in self.movement_instructions.iter().cycle() {
            steps += 1;
            for (starting_node, current_node) in zip(starting_nodes.iter(), current_nodes.iter_mut()) {
                if cycle_repetition.get(starting_node) != None {
                    continue;
                }
                *current_node = self.get_next(current_node, direction).unwrap();
                if Map::is_ending_node(current_node) {
                    cycle_repetition.insert(starting_node.to_string(), steps);
                }
                
            }
            if cycle_repetition.len() == num_of_starting_nodes {    // All endpoints have been found
                break;
            }
        }

        if let Some(lcm) = lcm_multiple(cycle_repetition.values().map(|value| *value)) {
            return lcm;
        }

        return 0;
    }

    pub fn load_from_file<'a>(file_name: &str) -> Map {
        let mut map = Map { graph: HashMap::new(), movement_instructions: Vec::new(), root: "AAA".to_string() };
        if let Ok(file_contents) = fs::read_to_string(file_name) {
            let mut lines = file_contents.lines();


            let movement_instructions_line = lines.next().unwrap();

            for movement_instruction_char in movement_instructions_line.as_bytes() {
                if let Ok(movement_instruction) = MovementInstruction::try_from(*movement_instruction_char as char) {
                    map.movement_instructions.push(movement_instruction);
                }
            }

            lines.next();

            for line in lines {
                match line.split(" = ").collect::<Vec<&str>>()[..2] {
                    [key_root, children] => {
                        let children_keys = children.split(", ").map(|potential_key| potential_key.replace(|character: char| !character.is_alphanumeric(), ""));
                        match &children_keys.collect::<Vec<String>>()[..2] {
                            [left, right] => {
                                map.graph.insert(key_root.to_string(), [left.to_owned(), right.to_owned()] );
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }

        }
        return map;
    }
}

fn main() {
    let map: Map = Map::load_from_file(FILE_NAME);
    println!("{:#?}", map);
    println!("{} steps to reach ZZZ", map.traverse_map());

    let map: Map = Map::load_from_file("test_input3.txt");
    map.traverse_map_as_ghost();
}

#[cfg(test)]
mod test {

    use crate::{Map, gcd};

    // #[test]
    // fn test_file_load() {
    //     let map: Map = Map::load_from_file("test_input.txt");
    // }

    #[test]
    fn test_traversal() {
        let map: Map = Map::load_from_file("test_input.txt");
        assert_eq!(map.traverse_map(), 2);
        let map: Map = Map::load_from_file("test_input2.txt");
        assert_eq!(map.traverse_map(), 6);
    }

    #[test]
    fn test_traversal_ghost() {
        let map: Map = Map::load_from_file("test_input3.txt");
        assert_eq!(map.traverse_map_as_ghost(), 6);
    }

    #[test]
    fn test_get_starting_nodes() {
        let map: Map = Map::load_from_file("test_input.txt");
        assert_eq!(map.get_starting_nodes(), vec!["AAA"]);
        let map: Map = Map::load_from_file("test_input2.txt");
        assert_eq!(map.get_starting_nodes(), vec!["AAA"]);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), Some(6));
        assert_eq!(gcd(42, 56), Some(14));
        assert_eq!(gcd(2, 0), Some(2));
        assert_eq!(gcd(1, 1), Some(1));
    }
}

