//--------------------------------------------------------------------------------
// Day 17: Clumsy Crucible
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::{collections::{HashMap, BinaryHeap, HashSet}, vec, cmp::Reverse};

const FILE_NAME: &'static str = "input.txt";

#[derive(Debug, PartialEq)]
struct City {
    map: Vec<Vec<u16>>,
    width: isize,
    height: isize
}

fn manhattan_distance(a: (isize, isize), b: (isize, isize)) -> usize {
    return a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
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
                        .map(|character| (character as char).to_digit(10).unwrap().try_into().unwrap())
                        .collect::<Vec<u16>>()
                );
            }
        }

        let height = map.len() as isize;
        let width = map[0].len() as isize;

        City { map, width, height  }
    }


    pub fn dijkstra_2(&self, minimum_movement: Option<isize>, maximum_movement: Option<isize>) -> u16 {
        let goal = (self.width - 1, self.height - 1);
        let mut costs: HashMap<(isize, isize, (isize, isize), u8), u16> = HashMap::new();
        let mut frontier: BinaryHeap<(Reverse<u16>, (isize, isize, (isize, isize), u8))> = BinaryHeap::from([(Reverse(0), (0, 0, (0, 0), 0))]);

        while let Some((Reverse(cost), (current_x, current_y, previous_dir, num_of_steps_in_same_direction))) = frontier.pop() {
            if (current_x, current_y) == goal {
                println!("found!");
                return cost;
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {

                if (-previous_dir.0, -previous_dir.1) == (dx, dy) {
                    continue;
                }

                if num_of_steps_in_same_direction >= maximum_movement.unwrap_or(0).try_into().unwrap_or(0) {
                    continue;
                }

                let mut next_num_of_steps_in_the_same_direction = 0;
                if previous_dir == (dx, dy) {
                    next_num_of_steps_in_the_same_direction = num_of_steps_in_same_direction + 1  
                }
                let next = (current_x + dx, current_y + dy, (dx, dy), next_num_of_steps_in_the_same_direction);
                let (next_x, next_y, (_, _), _) = next;

                if next_x >= self.width || next_y >= self.height || next_x < 0 || next_y < 0 {
                    continue;
                }

                let new_cost = cost + self.map[next_y as usize][next_x as usize];
                if !costs.contains_key(&next) || new_cost < *costs.get(&next).unwrap() {

                    if num_of_steps_in_same_direction < minimum_movement.unwrap_or(0).try_into().unwrap_or(0) {
                        continue;
                    }
                    costs.insert(next, new_cost);
                    let priority = Reverse(new_cost);
                    let key = (priority, next);
                    frontier.push(key);
                }
            }
        }
        unreachable!()
    }

    // Used Axel Lindeberg's solution because my Dijkstra implementation wouldn't work
    // https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/17.rs

    pub fn dijkstra(&self, minimum_movement: Option<isize>, maximum_movement: Option<isize>) -> u16 {
        let goal = (self.width - 1, self.height - 1);

        let mut costs: HashMap<(isize, isize, (isize, isize)), u16> = HashMap::new();
        let mut frontier: BinaryHeap<(Reverse<u16>, (isize, isize, (isize, isize)))> = BinaryHeap::from([(Reverse(0), (0, 0, (0, 0)))]);

        while let Some((Reverse(current_cost), (current_x, current_y, (current_dx, current_dy)))) = frontier.pop() {
            if (current_x, current_y) == goal {
                return current_cost;
            }

            if costs.get(&(current_x, current_y, (current_dx, current_dy))).is_some_and(|&stored_cost| stored_cost < current_cost) {
                continue;
            }

            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if (dx, dy) == (current_dx, current_dy) || (-dx, -dy) == (current_dx, current_dy) {
                    continue;
                }

                let mut next_cost = current_cost;

                for skip in 1..=maximum_movement.unwrap_or(3) {
                    let next_x = current_x + (dx * skip);
                    let next_y = current_y + (dy * skip);

                    if next_x >= self.width || next_x < 0 || next_y >= self.height || next_y < 0 {
                        continue;
                    }

                    next_cost += self.map[next_y as usize][next_x as usize];

                    if skip < minimum_movement.unwrap_or(1) {
                        continue;
                    }
                    
                    let next = (next_x, next_y, (dx, dy));
                    if !costs.contains_key(&next) || next_cost < *costs.get(&next).unwrap() {
                        costs.insert(next, next_cost);
                        frontier.push((Reverse(next_cost), next));
                    }
                }
            }
        }
        unreachable!()
    }
}


fn main() {
    println!("Hello, world!");
    let city = City::load_from_file(FILE_NAME);
    println!("min heat loss: {:?}", city.dijkstra(None, None));
    println!("min heat loss: {:?}", city.dijkstra(Some(4), Some(10)));
    println!("min heat loss: {:?}", city.dijkstra_2(None, None));
    println!("min heat loss: {:?}", city.dijkstra_2(Some(4), Some(10)));
}

#[cfg(test)]
mod tests {

    use crate::City;

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
        let min_heat = city.dijkstra(None, None);
        // let mut valid_path: HashSet<(isize, isize)> = HashSet::new();
        // assert!(path.contains_key(&(city.width - 1, city.height - 1)));

        // let mut heat_loss = 0;
        // let mut current_position: Option<(isize, isize)> = path.get(&(city.width - 1, city.height - 1)).cloned();

        // while current_position.is_some_and(|pos| pos != (0, 0)) {
        //     valid_path.insert(current_position.unwrap());
        //     let current_position_unwrapped = current_position.clone().unwrap();
        //     let current_coordinate = current_position_unwrapped;
        //     heat_loss += city.map[current_coordinate.1 as usize][current_coordinate.0 as usize];
        //     current_position = path.get(&current_position_unwrapped).cloned();
        // }

        // for i in 0..city.height {
        //     for j in 0..city.width {
        //         match valid_path.get(&(j, i)) {
        //             None => {
        //                 print!("{}", city.map[i as usize][j as usize])
        //             },
        //             Some(_) => {
        //                 print!("-");
        //             }
        //         }
        //     }
        //     println!();
        // }

        assert_eq!(min_heat, 102)
    }
}
