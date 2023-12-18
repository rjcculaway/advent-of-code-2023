//--------------------------------------------------------------------------------
// Day 13: Point of Incidence
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::{fs, iter::zip};

const FILE_NAME: &'static str = "input.txt";

#[derive(Debug)]
struct Terrain {
    grid: Vec<Vec<bool>>
}

impl Terrain {
    pub fn load_from_file(file_name: &str) -> Vec<Self> {
        let mut terrains: Vec<Self> = Vec::new();

        if let Ok(file_contents) = fs::read_to_string(file_name) {
            let lines = file_contents.lines();
            let mut grid_buffer: Vec<Vec<bool>> = Vec::new();
            for line in lines {
                // println!("{}", line);
                if line.len() > 0 {
                    let row = line.as_bytes()
                                .iter()
                                .map(|byte| 
                                    match byte.clone() as char {
                                        '#' => true,
                                        '.' => false,
                                        _ => unreachable!()
                                    }
                                );
                    grid_buffer.push(row.collect());
                    // println!("{:?}", grid_buffer);
                } else {
                    terrains.push(Self { grid: Vec::from_iter(grid_buffer.drain(..)) });
                    // println!("drained");
                }
            }
            terrains.push(Self { grid: Vec::from_iter(grid_buffer.drain(..)) });
        }
        terrains
    }
        
    pub fn detect_reflection(&self, reflection_difference: Option<u32>) -> (Option<usize>, Option<usize>) {
        let reflection_difference = reflection_difference.unwrap_or(0);
        let mut reflection_y: Option<usize> = None;
        let mut reflection_x: Option<usize> = None;

        let mut y: usize = 1;

        while y < self.grid.len() {
            // let iterator_length = y.min(self.grid.len() - y);

            let pairs = zip((0..y).rev(), (y)..self.grid.len());

            let difference: u32 = pairs.filter_map(|(up_i, down_i)| {

                let mut row_difference: u32 = 0;
                let Some(up) = self.grid.get(up_i) else {
                    return None;
                };
                let Some(down) = self.grid.get(down_i) else {
                    return None;
                };

                for (upper_symbol, lower_symbol) in zip(up, down) {
                    if upper_symbol ^ lower_symbol {
                        row_difference += 1;
                    }
                }

                return Some(row_difference);
            })
            .reduce(|prev, next| prev + next)
            .unwrap_or(0);

            // If all pairs mirror each other
            if difference == reflection_difference {
                reflection_y = Some(y);
            }

            y += 1;
        }

        let mut x = 1;

        while x < self.grid[0].len() {
            
            let pairs = zip((0..x).rev(), (x)..self.grid[0].len());
            
            let difference: u32 = pairs.filter_map(|(up_i, down_i)| {

                let mut col_difference: u32 = 0;
                let up = self.grid.iter().map(|row| row.get(up_i));
                let down = self.grid.iter().map(|row| row.get(down_i));

                for (upper_symbol_option, lower_symbol_option) in zip(up, down) {
                    let Some(upper_symbol) = upper_symbol_option else {
                        return None;
                    };
                    let Some(lower_symbol) = lower_symbol_option else {
                        return None;
                    };

                    if upper_symbol ^ lower_symbol {
                        col_difference += 1;
                    }
                }

                return Some(col_difference);
            })
            .reduce(|prev, next| prev + next)
            .unwrap_or(0);

            // If all pairs mirror each other
            if difference == reflection_difference {
                reflection_x = Some(x);
            }

            x += 1;
        }

        (reflection_x, reflection_y)
    }

    pub fn summarize_reflections(terrains: &Vec<Terrain>) -> u64 {
        
        terrains
            .iter()
            .map(|terrain| {
                let reflections = terrain.detect_reflection(Some(0));
                let vertical_reflection_value: u64 = match reflections.0 {
                    Some(x) => x.try_into().unwrap_or(0),
                    None => 0,
                };
                let horizontal_reflection_value: u64 = match reflections.1 {
                    Some(y) => y.try_into().unwrap_or(0) * 100,
                    None => 0,
                };

                return vertical_reflection_value + horizontal_reflection_value;
            })
            .sum()

    }

    pub fn summarize_reflections_with_smudge(terrains: &Vec<Terrain>) -> u64 {
        
        terrains
            .iter()
            .map(|terrain| {
                let reflections = terrain.detect_reflection(Some(1));
                let vertical_reflection_value: u64 = match reflections.0 {
                    Some(x) => x.try_into().unwrap_or(0),
                    None => 0,
                };
                let horizontal_reflection_value: u64 = match reflections.1 {
                    Some(y) => y.try_into().unwrap_or(0) * 100,
                    None => 0,
                };

                return vertical_reflection_value + horizontal_reflection_value;
            })
            .sum()

    }

}

fn main() {
    println!("Hello, world!");
    let terrains: Vec<Terrain> = Terrain::load_from_file(FILE_NAME);
    // println!("{:?}", terrains);
    // for terrain in terrains {
    //     println!("{:?}", terrain.detect_reflection());
    // }
    println!("summary: {}", Terrain::summarize_reflections(&terrains));
    println!("summary (with smudges): {}", Terrain::summarize_reflections_with_smudge(&terrains));
}

#[cfg(test)]
mod test {
    use crate::Terrain;
    use std::iter::zip;

    #[test]
    fn test_detect_reflection() {
        let terrains = Terrain::load_from_file("test_input.txt");
        let correct_answers: [(Option<usize>, Option<usize>); 2] = [(Some(5), None), (None, Some(4))];
        for (terrain, correct_answer) in zip(terrains, correct_answers) {
            assert_eq!(terrain.detect_reflection(Some(0)), correct_answer);
        }
    }

    #[test]
    fn test_fix_smudge_and_detect_reflection() {
        let terrains = Terrain::load_from_file("test_input.txt");
        let correct_answers: [(Option<usize>, Option<usize>); 2] = [(None, Some(3)), (None, Some(1))];
        for (terrain, correct_answer) in zip(terrains, correct_answers) {
            assert_eq!(terrain.detect_reflection(Some(1)), correct_answer);
        }
    }

    #[test]
    fn test_summarize_reflections() {
        let terrains = Terrain::load_from_file("test_input.txt");
        assert_eq!(Terrain::summarize_reflections(&terrains), 405);
    }
}
