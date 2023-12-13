//--------------------------------------------------------------------------------
// Day 08: Haunted Wasteland
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::{fs, collections::HashMap};

const FILE_NAME: &str = "test_input.txt";

#[derive(Debug)]
enum MovementInstruction {
    LEFT,
    RIGHT
}

impl From<char> for MovementInstruction {
    fn from(value: char) -> Self {
        match value {
            'L' => MovementInstruction::LEFT,
            'R' => MovementInstruction::RIGHT,
            _ => panic!()
        }
    }
}

#[derive(Debug)]
struct Node {
    key: String
}

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<String, Vec<Node>>,
    movement_instructions: Vec<MovementInstruction>
}

impl Graph {
    pub fn load_from_file<'a>(file_name: &str) -> Graph {
        let mut graph = Graph { adjacency_list: HashMap::new(), movement_instructions: Vec::new() };
        if let Ok(file_contents) = fs::read_to_string(file_name) {
            let mut lines = file_contents.lines();


            let movement_instructions_line = lines.next().unwrap();

            for movement_instruction_char in movement_instructions_line.as_bytes() {
                graph.movement_instructions.push((*movement_instruction_char))
            }

            lines.next();

            for line in lines {
                println!("{}", line);
                match line.split(" = ").collect::<Vec<&str>>()[..2] {
                    [key_root, children] => {
                        let children_keys = children.split(", ").map(|potential_key| return Node { key: potential_key.replace(|character: char| !character.is_alphabetic(), "") });
                        match children_keys.collect::<Vec<Node>>() {
                            children_keys => {
                                graph.adjacency_list.insert(key_root.to_string(), children_keys );
                            },
                        }
                    },
                    _ => {}
                }
            }

        }
        return graph;
    }
}

fn main() {
    let graph: Graph = Graph::load_from_file("test_input.txt");
    println!("{:#?}", graph);
}

#[cfg(test)]
mod test {

    use crate::Graph;

    #[test]
    fn test_file_load() {
        let graph: Graph = Graph::load_from_file("test_input.txt");
        println!("{:#?}", graph);
    }
}

