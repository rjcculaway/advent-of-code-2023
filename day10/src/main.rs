//--------------------------------------------------------------------------------
// Day 10: Pipe Maze
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::{fmt::Display, fs, collections::{VecDeque, HashMap, HashSet}};

const FILE_NAME: &'static str = "input.txt";

#[derive(Debug, Clone, Copy)]
enum Movement {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone, Copy)]
enum Legend {
    VerticalPipe,
    HorizontalPipe,
    BottomLeft,
    BottomRight,
    TopRight,
    TopLeft,
    Ground,
    Start
}

impl From<Legend> for char {
    fn from(value: Legend) -> Self {
        match value {
            Legend::VerticalPipe => '|',
            Legend::HorizontalPipe => '-',
            Legend::BottomLeft => 'L',
            Legend::BottomRight => 'J',
            Legend::TopRight => '7',
            Legend::TopLeft => 'F',
            Legend::Ground => '.',
            Legend::Start => 'S',
        }
    }
}

impl Into<Legend> for char {
    fn into(self) -> Legend {
        match self {
            '|' => Legend::VerticalPipe,
            '-' => Legend::HorizontalPipe,
            'L' => Legend::BottomLeft,
            'J' => Legend::BottomRight,
            '7' => Legend::TopRight,
            'F' => Legend::TopLeft,
            'S' => Legend::Start,
            '.' | _ => Legend::Ground,
        }
    }
}

impl Display for Legend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

struct Map {
    grid: Vec<Vec<Legend>>,
    width: u32,
    height: u32,
    start: (u32, u32)
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match writeln!(f, "Map ({}×{}):", self.width, self.height) {
            Ok(_) => {},
            Err(err) => { return Err(err) }
        }
        for row in &self.grid {
            for cell in row {
                match write!(f, "{cell}") {
                    Ok(_) => {}
                    Err(error) => {return Err(error);}
                }
            }
            match writeln!(f) {
                Ok(_) => {},
                Err(error) => { return Err(error); },
            }
        }
        match writeln!(f, "Start: ({}, {})", self.start.0, self.start.1) {
            Ok(_) => {},
            Err(err) => { return Err(err) }
        }
        Ok(())
    }
}

impl Map {
    pub fn load_from_file(file_name: &str) -> Option<Map> {
        let Ok(file_contents) = fs::read_to_string(file_name) else {
            return None;
        };

        let chars = file_contents.lines()
                                                                    .map(|line| line.as_bytes()
                                                                                        .iter()
                                                                                        .map(|byte| (*byte as char)
                                                                                        .into())
                                                                                    .collect::<Vec<Legend>>());
        let grid: Vec<Vec<Legend>> = Vec::from_iter(chars);
        let mut start: (u32, u32) = (0, 0); 

        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Legend::Start => {start.0 = j as u32; start.1 = i as u32},
                    _ => {}
                }
            }
        }

        let Ok(height) = grid.len().try_into() else {
            return None;
        };
        let Some(row) = grid.get(0) else {
            return None;
        };
        let Ok(width) = row.len().try_into() else {
            return None;
        };

        Some(Map { grid, width, height, start })
    }

    fn apply_movement(&self, current_position: (u32, u32), movement: &Movement) -> Option<(u32, u32)> {
        match movement {
            Movement::UP => {
                match u32::checked_sub(current_position.1, 1) {
                    Some(new_y) => {
                        return Some((current_position.0, new_y));
                    },
                    None => { return None },
                }
            },
            Movement::DOWN => {
                match u32::checked_add(current_position.1, 1) {
                    Some(new_y) => {
                        return Some((current_position.0, new_y));
                    },
                    None => { return None },
                }
            },
            Movement::LEFT => {
                match u32::checked_sub(current_position.0, 1) {
                    Some(new_x) => {
                        return Some((new_x, current_position.1));
                    },
                    None => { return None },
                }
            },
            Movement::RIGHT => {
                match u32::checked_add(current_position.0, 1) {
                    Some(new_x) => {
                        return Some((new_x, current_position.1));
                    },
                    None => { return None },
                }
            },
        }
    }

    fn access_cell(&self, position: (u32, u32)) -> Option<&Legend> {
        let Some(row) = self.grid.get(position.1 as usize) else {
            return None;
        };
        return row.get(position.0 as usize);
    }

    fn moveable_or_none(&self, current_position: (u32, u32), movement: &Movement) -> Option<(u32, u32)> {
        let Some(next_position) = self.apply_movement(current_position, &movement) else {
            return None;
        };



        // Check if pipe connects
        match movement {
            Movement::UP => {
                let Some(next_cell) = self.access_cell(next_position) else {
                    return None;
                };
                let Some(current_cell) = self.access_cell(current_position) else {
                    return None;
                };

                match (current_cell, next_cell) {
                    (Legend::VerticalPipe, Legend::VerticalPipe) | 
                    (Legend::VerticalPipe, Legend::TopRight) |
                    (Legend::VerticalPipe, Legend::TopLeft) |
                    (Legend::BottomLeft, Legend::VerticalPipe) |
                    (Legend::BottomLeft, Legend::TopRight) |
                    (Legend::BottomLeft, Legend::TopLeft) |
                    (Legend::BottomRight, Legend::VerticalPipe) |
                    (Legend::BottomRight, Legend::TopRight) |
                    (Legend::BottomRight, Legend::TopLeft) |
                    (Legend::Start, Legend::VerticalPipe) |
                    (Legend::Start, Legend::TopRight) |
                    (Legend::Start, Legend::TopLeft) => {
                        return Some(next_position);
                    },
                    _ => { return None; }
                }
            },
            Movement::DOWN => {
                let Some(next_cell) = self.access_cell(next_position) else {
                    return None;
                };
                let Some(current_cell) = self.access_cell(current_position) else {
                    return None;
                };

                match (current_cell, next_cell) {
                    (Legend::VerticalPipe, Legend::VerticalPipe) | 
                    (Legend::VerticalPipe, Legend::BottomRight) |
                    (Legend::VerticalPipe, Legend::BottomLeft) |
                    (Legend::TopLeft, Legend::VerticalPipe) |
                    (Legend::TopLeft, Legend::BottomRight) |
                    (Legend::TopLeft, Legend::BottomLeft) |
                    (Legend::TopRight, Legend::VerticalPipe) |
                    (Legend::TopRight, Legend::BottomRight) |
                    (Legend::TopRight, Legend::BottomLeft) |
                    (Legend::Start, Legend::VerticalPipe) |
                    (Legend::Start, Legend::BottomRight) |
                    (Legend::Start, Legend::BottomLeft) => {
                        return Some(next_position);
                    },
                    _ => { return None; }
                }
            },
            Movement::LEFT => {
                let Some(next_cell) = self.access_cell(next_position) else {
                    return None;
                };
                let Some(current_cell) = self.access_cell(current_position) else {
                    return None;
                };

                match (current_cell, next_cell) {
                    (Legend::HorizontalPipe, Legend::HorizontalPipe) |
                    (Legend::HorizontalPipe, Legend::BottomLeft) |
                    (Legend::HorizontalPipe, Legend::TopLeft) |
                    (Legend::BottomRight, Legend::HorizontalPipe) |
                    (Legend::BottomRight, Legend::BottomLeft) |
                    (Legend::BottomRight, Legend::TopLeft) |
                    (Legend::TopRight, Legend::HorizontalPipe) |
                    (Legend::TopRight, Legend::BottomLeft) |
                    (Legend::TopRight, Legend::TopLeft) |
                    (Legend::Start, Legend::BottomLeft) |
                    (Legend::Start, Legend::HorizontalPipe) => {
                        return Some(next_position);
                    },
                    _ => { return None; }
                }
            },
            Movement::RIGHT => {
                let Some(next_cell) = self.access_cell(next_position) else {
                    return None;
                };
                let Some(current_cell) = self.access_cell(current_position) else {
                    return None;
                };

                match (current_cell, next_cell) {
                    (Legend::HorizontalPipe, Legend::HorizontalPipe) |
                    (Legend::HorizontalPipe, Legend::BottomRight) |
                    (Legend::HorizontalPipe, Legend::TopRight) |
                    (Legend::BottomLeft, Legend::HorizontalPipe) |
                    (Legend::BottomLeft, Legend::BottomRight) |
                    (Legend::BottomLeft, Legend::TopRight) |
                    (Legend::TopLeft, Legend::HorizontalPipe) |
                    (Legend::TopLeft, Legend::BottomRight) |
                    (Legend::TopLeft, Legend::TopRight) |
                    (Legend::Start, Legend::BottomRight) |
                    (Legend::Start, Legend::HorizontalPipe) => {
                        return Some(next_position);
                    },
                    _ => { return None; }
                }
            },
        }
    }

    /**
     * Performs breadth first search. Gets the distance of each vertex to the source.
     */
    fn bfs_distance(&self) -> HashMap<(u32, u32), u32> {

        let possible_movements: [Movement; 4] = [Movement::UP, Movement::DOWN, Movement::LEFT, Movement::RIGHT];

        let mut frontier: VecDeque<(u32, u32)> = VecDeque::from([self.start.clone()]);
        let mut visited_distance: HashMap<(u32, u32), u32> = HashMap::from([(self.start.clone(), 0_u32)]);

        while !frontier.is_empty() {
            let current: (u32, u32) = frontier.pop_front().unwrap();
            let adjacents = possible_movements.iter().filter_map(| movement | self.moveable_or_none(current, movement));

            for adjacent in adjacents {
                if !visited_distance.contains_key(&adjacent) {
                    frontier.push_back(adjacent);
                    visited_distance.insert(adjacent.clone(), *visited_distance.get(&current).unwrap() + 1);
                }
            }
        }
        
        return visited_distance;
    }

    /**
     * Performs breadth first search. Gets the sequence of steps in the loop.
     */
    fn bfs_loop(&self) -> HashMap<(u32, u32), u32> {

        let possible_movements: [Movement; 4] = [Movement::UP, Movement::DOWN, Movement::LEFT, Movement::RIGHT];

        let mut frontier: VecDeque<(u32, u32)> = VecDeque::from([self.start.clone()]);
        let mut visited_distance: HashMap<(u32, u32), u32> = HashMap::from([(self.start.clone(), 0_u32)]);

        while !frontier.is_empty() {
            let current: (u32, u32) = frontier.pop_front().unwrap();
            let adjacents = possible_movements.iter().filter_map(| movement | self.moveable_or_none(current, movement));

            for adjacent in adjacents {
                if !visited_distance.contains_key(&adjacent) {
                    frontier.push_back(adjacent);
                    visited_distance.insert(adjacent.clone(), *visited_distance.get(&current).unwrap() + 1);
                    break;
                }
            }
        }
        
        return visited_distance;
    }

    fn dfs_ordered_by_traversal(&self) -> HashSet<(u32, u32)> {
        let possible_movements: [Movement; 4] = [Movement::UP, Movement::DOWN, Movement::LEFT, Movement::RIGHT];

        let mut frontier: VecDeque<(u32, u32)> = VecDeque::from([self.start.clone()]);
        let mut visited_distance: HashSet<(u32, u32)> = HashSet::from([self.start.clone()]);
        // let mut visited: Vec<(u32, u32)> = vec![self.start.clone()];

        while !frontier.is_empty() {
            let current: (u32, u32) = frontier.pop_front().unwrap();
            let adjacents = possible_movements.iter().filter_map(| movement | self.moveable_or_none(current, movement));

            for adjacent in adjacents {
                if !visited_distance.contains(&adjacent) {
                    frontier.push_front(adjacent);
                    visited_distance.insert(adjacent.clone());
                    // visited.push(adjacent.clone());
                }
            }
        }
        
        return visited_distance;
    }

    // fn print_bfs_by_distance(&self, visited_distance: &HashMap<(u32, u32), u32>) {
    //     for i in 0..self.height {
    //         for j in 0..self.width {
    //             match visited_distance.get(&(j, i)) {
    //                 None => print!(".\t"),
    //                 Some(distance) => print!("{distance}\t"),
    //             }
    //         }
    //         println!("");
    //     }
    // }

    // To get the area, we use the Shoelace formula: https://en.wikipedia.org/wiki/Pick%27s_theorem
    fn shoelace(mut vertices: Vec<(u32, u32)>) -> i64 {
        let mut area: i64 = 0;

        vertices.push(vertices.get(0).unwrap().clone());
        println!("{:?}", vertices);
        for pair in vertices.windows(2) {
            match pair {
                [left, right] => {
                    area += (right.0 as i64 - left.0 as i64) * (right.1 as i64 + left.1 as i64)
                },
                _ => {}
            }
        }
        println!("2*area: {area}");
        return i64::abs(area / 2);
    }

    // Pick's theorem, rearranged formula to get the interior points
    fn picks_theorem_interior(area: i64, num_of_vertices: i64) -> i64 {
        let interior = area - (num_of_vertices / 2) + 1;
        return interior;
    }
    
    fn get_interior_area(&self) -> i64 {
        let vertices_step = self.bfs_loop();
        let mut vertices = Vec::from_iter(vertices_step.keys().map(|position| position.clone()));
        vertices.sort_by(|a, b| u32::cmp(vertices_step.get(a).unwrap(), vertices_step.get(b).unwrap()));
        let num_of_vertices = vertices.len();
        let total_area = Map::shoelace(vertices);
        let interior = Map::picks_theorem_interior(total_area, num_of_vertices as i64);

        return interior;
    }

}

fn main() {
    let Some(map) = Map::load_from_file(FILE_NAME) else {
        return;
    };
    let distances = map.bfs_distance();
    println!("{:#?}", distances.values().max());
    println!("{:?}", map.dfs_ordered_by_traversal());
    let area = map.get_interior_area();
    println!("Area: {:#?}", area);
}

#[cfg(test)]

mod test {

    use crate::Map;

    #[test]
    fn test_bfs_1() {
        let Some(map) = Map::load_from_file("test_input.txt") else {
            panic!();
        };
        let distances = map.bfs_distance();
        // map.print_bfs_by_distance(&distances);
        assert_eq!(*distances.values().max().unwrap(), 4);
    }

    #[test]
    fn test_bfs_2() {
        let Some(map) = Map::load_from_file("test_input2.txt") else {
            panic!();
        };
        // let distances = map.bfs_distance();
        // map.print_bfs_by_distance(&distances);
        assert_eq!(*map.bfs_distance().values().max().unwrap(), 8);
    }

    #[test]
    fn test_area() {
        let Some(map) = Map::load_from_file("test_input3.txt") else {
            panic!();
        };
        // let distances = map.bfs_loop();
        // map.print_bfs_by_distance(&distances);
        assert_eq!(map.get_interior_area(), 4);
    }

    #[test]
    fn test_area2() {
        let Some(map) = Map::load_from_file("test_input.txt") else {
            panic!();
        };
        // let distances = map.bfs_loop();
        // map.print_bfs_by_distance(&distances);
        assert_eq!(map.get_interior_area(), 1);
    }
}
