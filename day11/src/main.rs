//--------------------------------------------------------------------------------
// Day 11: Cosmic Expansion
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

const FILE_NAME: &'static str = "input.txt";

use std::collections::{BTreeSet, HashSet};
use std::fs;

#[derive(Debug)]
enum Legend {
    Galaxy,
    Empty,
}

impl TryFrom<char> for Legend {
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Legend::Galaxy),
            '.' => Ok(Legend::Empty),
            _ => Err("Character is not a Legend."),
        }
    }

    type Error = &'static str;
}

impl From<Legend> for char {
    fn from(value: Legend) -> Self {
        match value {
            Legend::Galaxy => '#',
            Legend::Empty => '.',
        }
    }
}

#[derive(Debug)]
struct Map {
    // grid: Vec<Vec<Legend>>,
    empty_y: BTreeSet<u64>,
    empty_x: BTreeSet<u64>,
    galaxies: Vec<(u64, u64)>
}

fn manhattan_distance(a: (u64, u64), b: (u64, u64)) -> u64 {
    return u64::abs_diff(a.0, b.0) + u64::abs_diff(a.1, b.1);
}

impl Map {

    pub fn get_expanded_coordinate(&self, pos: (u64, u64), expansion_level: Option<u64>) -> (u64, u64) {
        let x_range = 0..pos.0;
        let y_range = 0..pos.1;

        let num_of_col_expansions = self.empty_x.range(x_range).count() as u64;
        let num_of_row_expansions = self.empty_y.range(y_range).count() as u64;

        // println!("expansion: {}, {}", num_of_col_expansions, num_of_row_expansions);
        return (pos.0 + num_of_col_expansions * (expansion_level.unwrap_or(2) - 1), pos.1 + num_of_row_expansions * (expansion_level.unwrap_or(2) - 1));
    }

    pub fn enumerate_combinations(&self) -> HashSet<(u64, u64)> {
        let num_of_galaxies = self.galaxies.len() as u64;
        let mut combinations: HashSet<(u64, u64)> = HashSet::new();

        for i in 0..num_of_galaxies {
            for j in 0..num_of_galaxies {
                if i == j { // A galaxy is not paired with itself
                    continue;
                }
                combinations.insert((i.min(j), i.max(j)));
            }
        }

        return combinations;
    }

    pub fn compute_galaxy_distance(&self, pair: (u64, u64), expansion_level: Option<u64>) -> Option<u64> {
        let a = self.galaxies.get(pair.0 as usize)?.clone();
        let b = self.galaxies.get(pair.1 as usize)?.clone();
        
        let expanded_a = self.get_expanded_coordinate(a, expansion_level);
        let expanded_b = self.get_expanded_coordinate(b, expansion_level);
        
        Some(manhattan_distance(expanded_a, expanded_b))
    }

    pub fn compute_sum_galaxy_pairs (&self) -> u64 {
        let galaxy_pairs = self.enumerate_combinations();
        return galaxy_pairs.into_iter()
                        .map(|pair| self.compute_galaxy_distance(pair, Some(2)).unwrap_or(0))
                        .reduce(|prev, curr| prev + curr)
                        .unwrap_or(0);
    }

    pub fn compute_sum_galaxy_pairs_million (&self) -> u64 {
        let galaxy_pairs = self.enumerate_combinations();
        return galaxy_pairs.into_iter()
                        .map(|pair| self.compute_galaxy_distance(pair, Some(1_000_000)).unwrap_or(0))
                        .reduce(|prev, curr| prev + curr)
                        .unwrap_or(0);
    }

    pub fn load_from_file(file_name: &str) -> Map {
        // let mut grid: Vec<Vec<Legend>> = vec![];
        let mut empty_y: BTreeSet<u64> = BTreeSet::new();
        let mut empty_x: BTreeSet<u64> = BTreeSet::new();
        let mut galaxies: Vec<(u64, u64)> = vec![];

        let mut empty_x_mask: Option<Vec<bool>> = None;

        if let Ok(file_contents) = fs::read_to_string(file_name) {
            for (i, line) in file_contents.lines().enumerate() {
                let legends: Vec<Legend> = line.as_bytes().iter()
                                            .map(|byte| (*byte as char).try_into().unwrap())
                                            .collect();

                if empty_x_mask.is_none() {
                    empty_x_mask = Some(vec![true; line.len()]);
                }

                let mut row_has_galaxy = false;
                for (j, legend) in legends.iter().enumerate() {
                    match legend {
                        Legend::Galaxy => {
                            row_has_galaxy = true;
                            empty_x_mask.as_deref_mut().unwrap()[j] = false;
                            galaxies.push((j.try_into().unwrap(), i.try_into().unwrap()));
                        },
                        Legend::Empty => {},
                    }
                }

                if !row_has_galaxy {
                    empty_y.insert(i.try_into().unwrap());
                }

                // grid.push(legends);
            }
        }

        for (i, column_mask) in empty_x_mask.unwrap().iter().enumerate() {
            if *column_mask {
                empty_x.insert(i.try_into().unwrap());
            }
        }

        return Map { empty_y, empty_x, galaxies }
    }
}

fn main() {
    println!("Hello, world!");
    let map: Map = Map::load_from_file(FILE_NAME);
    println!("Sum of distances: {}", map.compute_sum_galaxy_pairs());
    println!("Sum of distances (million expansion): {}", map.compute_sum_galaxy_pairs_million());
}

#[cfg(test)]

mod tests {
    use crate::Map;

    #[test]
    fn test_combinations() {
        let map = Map::load_from_file("test_input.txt");
        assert_eq!(map.enumerate_combinations().len(), 36);
    }

    #[test]
    fn test_galaxy_distance() {
        let map = Map::load_from_file("test_input.txt");
        let input_output: [((u64, u64), u64); 4] = [((5, 9), 9), ((1, 7), 15), ((3, 6), 17), ((8, 9), 5)];

        for ((a, b), expected_result) in input_output {
            assert_eq!(map.compute_galaxy_distance((a - 1, b - 1), Some(2)), Some(expected_result));
        }
    }

    #[test]
    fn test_sum_galaxy_pairs() {
        let map = Map::load_from_file("test_input.txt");
        println!("{:?}", map.enumerate_combinations());
        assert_eq!(map.compute_sum_galaxy_pairs(), 374);
    }
}
