//--------------------------------------------------------------------------------
// Day 13: Parabolic Reflector Dish
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::collections::HashMap;

const FILE_NAME: &'static str = "input.txt";

struct Map {
    grid: Vec<Vec<char>>
}

impl Map {
    pub fn load_from_file(file_name: &str) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();

        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            for line in file_contents.lines() {
                grid.push(
                    line.as_bytes()
                        .iter()
                        .map(|b| *b as char)
                        .collect()
                )
            }
        }

        Map { grid }
    }

    // Could've worked smarter and transposed the matrix for those hot cache gains, but I got lazy lol
    pub fn slide_north(&mut self) {
        let width: usize = self.grid[0].len();
        let height: usize = self.grid.len();
        let mut next_empty_space: Vec<usize> = vec![0].repeat(self.grid[0].len());

        for j in 0..width {
            for i in 0..height {
                if self.grid[i][j] == '.' {
                    next_empty_space[j] = i;
                    break;
                }
            }
        }
        
        for j in 0..width {
            let mut i = next_empty_space[j];
            while i < height {
                let cell = self.grid[i][j];
                match cell {
                    '#' => {
                        // Search for the next empty cell
                        while i < height {
                            if self.grid[i][j] == '.' {
                                next_empty_space[j] = i;
                                break;
                            }
                            i += 1;
                        }
                    }
                    'O' => {
                        let will_fall = self.grid.get(i - 1).is_some_and(| previous | {
                            match previous[j] {
                                '#' | 'O' => false,
                                '.' => true,
                                _ => unreachable!()
                            }
                        });

                        if will_fall {
                            self.grid[i][j] = '.';
                            self.grid[next_empty_space[j]][j] = 'O';
                            next_empty_space[j] = (next_empty_space[j] + 1).min(height - 1);
                        }
                    },
                    '.' => {},
                    _ => unreachable!()
                }
                i += 1
            }
        }
    }

    pub fn slide_south(&mut self) {
        let width: usize = self.grid[0].len();
        let height: usize = self.grid.len();
        let mut next_empty_space: Vec<usize> = vec![0].repeat(self.grid[0].len());

        for j in (0..width).rev() {
            for i in (0..height).rev() {
                if self.grid[i][j] == '.' {
                    next_empty_space[j] = i;
                    break;
                }
            }
        }
        
        for j in (0..width).rev() {
            let mut i = next_empty_space[j];
            loop {
                let cell = self.grid[i][j];
                match cell {
                    '#' => {
                        // Search for the next empty cell
                        loop {
                            if self.grid[i][j] == '.' {
                                next_empty_space[j] = i;
                                break;
                            }
                            let Some(next_i) = i.checked_sub(1) else {
                                break;
                            };
                            i = next_i;
                        }
                    }
                    'O' => {
                        let will_fall = self.grid.get(i + 1).is_some_and(| previous | {
                            match previous[j] {
                                '#' | 'O' => false,
                                '.' => true,
                                _ => unreachable!()
                            }
                        });

                        if will_fall {
                            self.grid[i][j] = '.';
                            self.grid[next_empty_space[j]][j] = 'O';
                            next_empty_space[j] = (next_empty_space[j] - 1).max(0);
                        }
                    },
                    '.' => {},
                    _ => unreachable!()
                }
                let Some(next_i) = i.checked_sub(1) else {
                    break;
                };
                i = next_i;
            }

            
        }
    }
    
    pub fn slide_west(&mut self) {
        let width: usize = self.grid[0].len();
        let height: usize = self.grid.len();
        let mut next_empty_space: Vec<usize> = vec![0].repeat(self.grid.len());

        for i in 0..height {
            for j in 0..width {
                if self.grid[i][j] == '.' {
                    next_empty_space[i] = j;
                    break;
                }
            }
        }
        
        for i in 0..height {
            let mut j = next_empty_space[i];
            while j < width {
                let cell = self.grid[i][j];
                match cell {
                    '#' => {
                        // Search for the next empty cell
                        while j < width {
                            if self.grid[i][j] == '.' {
                                next_empty_space[i] = j;
                                break;
                            }
                            j += 1;
                        }
                    }
                    'O' => {
                        let will_fall = self.grid.get(i).is_some_and(| previous | {
                            match previous[j - 1] {
                                '#' | 'O' => false,
                                '.' => true,
                                _ => unreachable!()
                            }
                        });

                        if will_fall {
                            self.grid[i][j] = '.';
                            self.grid[i][next_empty_space[i]] = 'O';
                            next_empty_space[i] = (next_empty_space[i] + 1).min(height - 1);
                        }
                    },
                    '.' => {},
                    _ => unreachable!()
                }
                j += 1
            }
        }
    }

    pub fn slide_east(&mut self) {
        let width: usize = self.grid[0].len();
        let height: usize = self.grid.len();
        let mut next_empty_space: Vec<usize> = vec![0].repeat(self.grid.len());

        for i in (0..height).rev() {
            for j in (0..width).rev() {
                if self.grid[i][j] == '.' {
                    next_empty_space[i] = j;
                    break;
                }
            }
        }
        
        for i in (0..height).rev() {
            let mut j = next_empty_space[i];
            loop {
                let cell = self.grid[i][j];
                match cell {
                    '#' => {
                        // Search for the next empty cell
                        loop {
                            if self.grid[i][j] == '.' {
                                next_empty_space[i] = j;
                                break;
                            }
                            let Some(next_j) = j.checked_sub(1) else {
                                break;
                            };
                            j = next_j;
                        }
                    }
                    'O' => {
                        let will_fall = self.grid.get(i).is_some_and(| previous | {
                            match previous[j + 1] {
                                '#' | 'O' => false,
                                '.' => true,
                                _ => unreachable!()
                            }
                        });

                        if will_fall {
                            self.grid[i][j] = '.';
                            self.grid[i][next_empty_space[i]] = 'O';
                            next_empty_space[i] = (next_empty_space[i] - 1).max(0);
                        }
                    },
                    '.' => {},
                    _ => unreachable!()
                }
                let Some(next_j) = j.checked_sub(1) else {
                    break;
                };
                j = next_j;
            }
        }

        // for row in &self.grid {
        //     for cell in row {
        //         print!("{cell}");
        //         // if next_empty_space[j] == i {
        //         //     print!("-");
        //         // } else {
        //         // }
        //     }
        //     println!()
        // }
    }
    
    pub fn compress_grid(&self) -> String {
        return self.grid.iter()
                .map(|row| row.iter().cloned().collect::<String>())
                .reduce(|prev, next| prev + &next).unwrap_or("".to_string());
    }

    // Idea to cache the state and check where the loop begins from 
    // Dr. Neil Smith: https://work.njae.me.uk/
    // I already encountered this pattern before so it was a bummer that I wasn't able to get it on my own.
    fn spin_cycle(&mut self, number_of_cycles: Option<u32>) {
        let number_of_cycles: u32 = number_of_cycles.unwrap_or(1000000000);
        let mut cache: HashMap<String, u32> = HashMap::new();
        for current_cycle in 0..number_of_cycles {
            self.slide_north();
            self.slide_west();
            self.slide_south();
            self.slide_east();

            let compressed_grid: String = self.compress_grid();
            if cache.contains_key(&compressed_grid) {
                println!("loop found");
                let loop_start = cache.get(&compressed_grid).unwrap();
                let period = current_cycle - loop_start;
    
                let remaining_cycles = (number_of_cycles - current_cycle - 1) % period;

                for _ in 0..remaining_cycles {
                    self.slide_north();
                    self.slide_west();
                    self.slide_south();
                    self.slide_east();
                }
                
                break;
            } else {
                cache.insert(compressed_grid, current_cycle);
            }

        }
    }

    fn compute_current_load(&self) -> u32 {
        let height: usize = self.grid.len();
        let mut total_load: u32 = 0;

        for (i, row) in self.grid.iter().enumerate() {
            let distance: u32 = (height - i).try_into().unwrap_or(0);
            // println!("{distance}");
            let number_of_rocks = row
                        .iter()
                        .filter(|character| **character == 'O')
                        .count().try_into().unwrap_or(0);
            total_load += number_of_rocks * distance;
        }

        total_load
    }

    
}

fn main() {
    // let mut map: Map = Map::load_from_file(FILE_NAME);
    // map.slide_east();
    // println!("total load: {}", map.compute_current_load());

    let mut map: Map = Map::load_from_file(FILE_NAME);
    map.spin_cycle(None);
    println!("total load: {}", map.compute_current_load());
}

#[cfg(test)]
mod test {
    use crate::Map;

    #[test]
    fn test_slide_north() {
        let mut map = Map::load_from_file("test_input.txt");
        map.slide_north();
        let correct_answer: Vec<Vec<char>> = vec![
                vec!['O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.'],
                vec!['O', 'O', '.', '.', '#', '.', '.', '.', '.', '#'],
                vec!['O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O'],
                vec!['O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
                vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
                vec!['.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O'],
                vec!['.', '.', 'O', '.', '.', '.', '.', '.', '.', '.'],
                vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
                vec!['#', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
        ];

        assert_eq!(map.grid, correct_answer);
    }

    #[test]
    fn test_compute_current_load() {
        let mut map = Map::load_from_file("test_input.txt");
        map.slide_north();
        let correct_answer = 136;

        assert_eq!(map.compute_current_load(), correct_answer);

        let mut map = Map::load_from_file("test_input.txt");
        map.spin_cycle(None);
        let correct_answer: u32 = 64;
        assert_eq!(map.compute_current_load(), correct_answer);
    }

    #[test]
    fn test_spin_cycle() {
        let mut map = Map::load_from_file("test_input.txt");
        map.spin_cycle(Some(1));
        let correct_answer: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', 'O', '#'],
            vec!['.', '.', '.', 'O', 'O', '#', '#', '.', '.', '.'],
            vec!['.', 'O', 'O', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', 'O', 'O', 'O', '#', '.'],
            vec!['.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#'],
            vec!['.', '.', '.', '.', 'O', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'O', 'O', 'O', 'O'],
            vec!['#', '.', '.', '.', 'O', '#', '#', '#', '.', '.'],
            vec!['#', '.', '.', 'O', 'O', '#', '.', '.', '.', '.'],
        ];

        assert_eq!(map.grid, correct_answer);
        map.spin_cycle(Some(1));

        let correct_answer: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', 'O', '#'],
            vec!['.', '.', '.', '.', '.', '#', '#', '.', '.', '.'],
            vec!['.', '.', 'O', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', 'O', 'O', 'O', '#', '.'],
            vec!['.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#'],
            vec!['.', '.', '.', '.', 'O', '#', '.', '.', '.', 'O'],
            vec!['.', '.', '.', '.', '.', '.', '.', 'O', 'O', 'O'],
            vec!['#', '.', '.', 'O', 'O', '#', '#', '#', '.', '.'],
            vec!['#', '.', 'O', 'O', 'O', '#', '.', '.', '.', 'O'],
        ];

        assert_eq!(map.grid, correct_answer);
        map.spin_cycle(Some(1));

        let correct_answer: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', 'O', '#'],
            vec!['.', '.', '.', '.', '.', '#', '#', '.', '.', '.'],
            vec!['.', '.', 'O', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', 'O', 'O', 'O', '#', '.'],
            vec!['.', 'O', '#', '.', '.', '.', 'O', '#', '.', '#'],
            vec!['.', '.', '.', '.', 'O', '#', '.', '.', '.', 'O'],
            vec!['.', '.', '.', '.', '.', '.', '.', 'O', 'O', 'O'],
            vec!['#', '.', '.', '.', 'O', '#', '#', '#', '.', 'O'],
            vec!['#', '.', 'O', 'O', 'O', '#', '.', '.', '.', 'O'],
        ];
        assert_eq!(map.grid, correct_answer);
    }
}
