//--------------------------------------------------------------------------------
// Day 17: Clumsy Crucible
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::{collections::{HashMap, BTreeSet}, vec};

const FILE_NAME: &'static str = "test_input.txt";

#[derive(Debug, Hash, Clone, Copy)]
struct PrioritizedPosition(Position, u16);

impl PartialEq for PrioritizedPosition {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&self.1)
    }
}

impl Eq for PrioritizedPosition {}

impl PartialOrd for PrioritizedPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.1.partial_cmp(&other.1);
    }
}

impl Ord for PrioritizedPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    coordinate: (isize, isize),
}

impl Position {
    fn manhattan_distance(&self, other: &Position) -> u16 {
        let self_coordinate: (isize, isize) = self.coordinate;
        let other_coordinate: (isize, isize) = other.coordinate;

        self_coordinate.0.abs_diff(other_coordinate.0) as u16 + self_coordinate.1.abs_diff(other_coordinate.1) as u16
    }
}

#[derive(Debug, PartialEq)]
struct City {
    map: Vec<Vec<u16>>,
    width: isize,
    height: isize
}

impl City {
    pub fn load_from_file(file_name: &str) -> City {
        let mut map: Vec<Vec<u16>> = vec![];

        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            for line in file_contents.lines() {
                map.push(
                    line
                        .as_bytes()
                        .iter()
                        .cloned()
                        .map(|character| character as u16 - 48)
                        .collect::<Vec<u16>>()
                );
            }
        }

        let height = map.len() as isize;
        let width = map[0].len() as isize;

        City { map, width, height  }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    pub fn get_vector(&self) -> (isize, isize) {
        match self {
            Direction::North => {
                (0, -1)
            },
            Direction::South => {
                (0, 1)
            },
            Direction::West => {
                (-1, 0)
            },
            Direction::East => {
                (1, 0)
            },
        }
    }
}

enum AgentState {
    MustTurn,
    FirstMovement,
    SecondMovement,
    ThirdMovement
}

struct Agent {
    current_state: AgentState,
    current_direction: Direction
}

impl Agent {
    pub fn new() -> Self {
        Agent { current_state: AgentState::MustTurn, current_direction: Direction::North }
    }

    fn next_state(&mut self) {
        match self.current_state {
            AgentState::MustTurn => {
                self.current_state = AgentState::FirstMovement;
            },
            AgentState::FirstMovement => {
                self.current_state = AgentState::SecondMovement;
            },
            AgentState::SecondMovement => {
                self.current_state = AgentState::ThirdMovement;
            },
            AgentState::ThirdMovement => {
                self.current_state = AgentState::MustTurn;
            },
        }
    }

    pub fn get_next_adjacent(&self, position: &Position) -> Vec<Direction> {
        // return vec![Direction::West, Direction::East, Direction::North, Direction::South];
        match self.current_state {
            AgentState::MustTurn => {
                match self.current_direction {
                    Direction::North | Direction::South => {
                        vec![Direction::West, Direction::East]
                    },
                    Direction::West | Direction::East => {
                        vec![Direction::North, Direction::South]
                    },
                }
            },
            _ => {
                match self.current_direction {
                    Direction::North | Direction::South => {
                        vec![self.current_direction, Direction::West, Direction::East]
                    },
                    Direction::West | Direction::East => {
                        vec![self.current_direction, Direction::North, Direction::South]
                    },
                }
            }
        }
    }
    
    // Technically A* but modified
    pub fn transport_cauldron(&mut self, city: &City, starting_position: Option<Position>, goal: Option<Position>) -> HashMap<Position, Position> {
        let starting_position = starting_position.unwrap_or(Position { coordinate: (0, 0) });
        let goal = goal.unwrap_or(Position { coordinate: (city.width - 1, city.height - 1) });
        
        let mut frontier: BTreeSet<PrioritizedPosition> = BTreeSet::from([PrioritizedPosition(starting_position, 0)]);
        let mut from_to: HashMap<Position, Position> = HashMap::new();
        let mut cost_to: HashMap<Position, u16> = HashMap::from([(starting_position, 0)]);

        while !frontier.is_empty() {
            let PrioritizedPosition(current, cozt) = frontier.pop_first().unwrap();
            self.next_state();

            println!("cost: {cozt}");

            if current.coordinate == goal.coordinate {
                println!("found");
                break;
            }
            
            let adjacents = self.get_next_adjacent(&current);
            let valid_adjacents = adjacents.iter()
                .filter_map(|direction| {
                    let vector = direction.get_vector();
                    let next_coordinate = (current.coordinate.0 + vector.0, current.coordinate.1 + vector.1);

                    if next_coordinate.1 >= city.height || next_coordinate.1 < 0 ||
                    next_coordinate.0 >= city.width || next_coordinate.0 < 0 {
                        return None;
                    }
                    
                    return Some((next_coordinate, direction));
                });

            for (adjacent, direction) in valid_adjacents {
                let position_adjacent: Position = Position { coordinate: adjacent };
                let heat = city.map[adjacent.1 as usize][adjacent.0 as usize];
                let new_cost = cost_to.get(&current).cloned().unwrap_or(0) + heat;
                
                if !cost_to.contains_key(&position_adjacent) || new_cost < *cost_to.get(&position_adjacent).unwrap() {
                    cost_to.insert(position_adjacent, new_cost);
                    let priority = new_cost + Position::manhattan_distance(&goal, &position_adjacent);
                    frontier.insert(PrioritizedPosition(position_adjacent, priority));
                    from_to.insert(position_adjacent, current);
                    self.current_direction = *direction;
                }
            }
        }
        from_to
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{City, Agent, Position};

    #[test]
    fn test_load_from_file() {
        let city: City = City::load_from_file("test_input.txt");
        assert_eq!(city, City { map: vec![
            vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
            vec![3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
            vec![3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
            vec![3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
            vec![4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
            vec![1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
            vec![4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
            vec![3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
            vec![4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
            vec![4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
            vec![1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
            vec![2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
            vec![4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
        ], width: 13, height: 13 })
    }

    #[test]
    fn test_transport_cauldron() {
        let city: City = City::load_from_file("test_input.txt");
        let mut agent: Agent = Agent::new();
        let path = agent.transport_cauldron(&city, None, None);
        println!("{} number of steps", path.len());

        let mut heat_loss = 0;
        let mut current_position: Option<Position> = path.get(&Position { coordinate: (city.width - 1, city.height - 1) }).cloned();

        while current_position.is_some() {
            let current_position_unwrapped = current_position.clone().unwrap();
            let current_coordinate = current_position_unwrapped.coordinate;
            heat_loss += city.map[current_coordinate.1 as usize][current_coordinate.0 as usize];
            current_position = path.get(&current_position_unwrapped).cloned();
        }

        for i in 0..city.height {
            for j in 0..city.width {
                match path.get(&(Position {coordinate: (j, i)})) {
                    None => {
                        print!("{}", city.map[i as usize][j as usize])
                    },
                    Some(_) => {
                        print!("-");
                    }
                }
            }
            println!();
        }

        assert_eq!(heat_loss, 102);
    }
}
