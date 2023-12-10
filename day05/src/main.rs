//--------------------------------------------------------------------------------
// Day 05: If You Give A Seed A Fertilizer
// Rene Jotham Culaway
// I hate this day!!!
// Part 2 based on Reddit user zuleyorker's answer: https://topaz.github.io/paste/#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 
//--------------------------------------------------------------------------------

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::fs;
use std::ops::Range;

use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;

const FILE_NAME: &str = "input.txt";

#[derive(Clone, Debug)]
struct RangeMap {
    start_src: u64,
    start_dst: u64,
    range: u64
}

impl RangeMap {
    pub fn convert_to_value(&self, key: u64) -> u64 {
        return self.start_dst + (key - self.start_src);
    }

    pub fn is_within_range(&self, value: &u64) -> bool {
        return (self.start_src..(self.start_src + self.range)).contains(&value);
    }
}

#[derive(Debug)]
struct AlmanacMap {
    map_label: String,
    map: Vec<RangeMap>
}

impl AlmanacMap {
    pub fn inverse(&self) -> AlmanacMap {
        let mut label_components = self.map_label.split("-to-").collect::<Vec<&str>>();
        label_components.reverse();
        let new_label = label_components.join("-to-");
        let mut inverse_map: Vec<RangeMap> = Vec::new();

        for inverted_range_map in self.map.iter().map(|range_map| return RangeMap {start_src: range_map.start_dst, start_dst: range_map.start_src, range: range_map.range}) {
            match inverse_map.binary_search_by_key(&inverted_range_map.start_src, |probe| return probe.start_src) {
                Ok(_) => {},
                Err(pos) => {
                    inverse_map.insert(pos, inverted_range_map);
                }
            }
        }
        return AlmanacMap { map_label: new_label, map: inverse_map };
    }
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {

    pub fn get_seeds_to_locations(&self) -> BinaryHeap<Reverse<u64>> {
        let mut seeds_to_locations: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
        
        for seed in &self.seeds {
            let mut current_transformation: u64 = *seed;
            
            for almanac_map in &self.maps {
                let mut applicable_mapping: Option<RangeMap> = None;
                for range_map in almanac_map.map.iter() {
                    if range_map.is_within_range(&current_transformation) {
                        applicable_mapping = Some(range_map.clone());
                        break;
                    }
                } 
                if applicable_mapping.is_some() {
                    current_transformation = applicable_mapping.unwrap().convert_to_value(current_transformation);
                }
            }
            seeds_to_locations.push(Reverse(current_transformation));
        }

        return seeds_to_locations;
    }

    fn has_inverse_mapping(inverse_maps: &Vec<AlmanacMap>, seed_ranges: &Vec<Range<u64>>, location: u64) -> bool {
        let mut current_transformation = location;
        for almanac_map in inverse_maps {
            match almanac_map.map.binary_search_by(|probe| {
                if current_transformation >= probe.start_src + probe.range {
                    return Less
                } else if current_transformation < probe.start_src {
                    return Greater
                } else {
                    return Equal;
                }
            }) {
                Ok(pos) => {
                    let new_transformation = almanac_map.map[pos].convert_to_value(current_transformation);
                    current_transformation = new_transformation;
                    
                },
                Err(_) => {}
            }
        }

        match seed_ranges.binary_search_by(|probe| {
            if probe.contains(&current_transformation) {
                return Equal
            } else if current_transformation < probe.start {
                return Greater
            } else {
                return Less
            }
        }) {
            Ok(_) => {
                println!("{} is mapped to {}.", location, current_transformation);
                true
            }
            Err(_) => {
                // println!("{} has no mapping.", location);
                false
            }
        }
    }

    pub fn get_minimum_location_from_seed_ranges(&self) -> u64 {

        let inverse_almanac_maps: Vec<AlmanacMap> = self.maps.iter().rev().map(|range_map| range_map.inverse()).collect();
        let mut seed_ranges: Vec<Range<u64>> = self.seeds.chunks(2).map(|chunk| {
            match chunk {
                [start, range] => {
                    return *start..(start+range);
                },
                _ => panic!()
            }
        }).collect();
        seed_ranges.sort_by_key(|range| { range.start });

        let mut current_location: u64 = 0;
        while !Almanac::has_inverse_mapping(&inverse_almanac_maps, &seed_ranges, current_location) {
            current_location += 1;
        }

        return current_location;
    }

    pub fn load_almanac_from_file(file_name: &str) -> Almanac {

        let mut almanac: Almanac = Almanac {
            seeds: Vec::new(),
            maps: Vec::new(),
        };

        if let Ok(file_contents) = fs::read_to_string(file_name) {
            let lines: Vec<&str> = file_contents.lines().collect();
            // Collect seeds
            let seeds_line = lines.get(0).unwrap();
            let seeds_space_separated = match seeds_line.split_at(7) {
                (_, value) => { value }
            };
            almanac.seeds.append(&mut seeds_space_separated.split(" ").map(|seed_str: &str| return seed_str.parse::<u64>().unwrap()).collect());
            
            let reading_start: usize = 2;

            let mut lines_iter = lines.iter().skip(reading_start).peekable();
            while lines_iter.peek().is_some() {
                let line = lines_iter.next().unwrap();
                // println!("{:?}", line.split_whitespace().next().unwrap());
                // Extract map name
                let mut almanac_map: AlmanacMap = AlmanacMap { 
                    map_label: line.split_whitespace().next().unwrap().to_string(), 
                    map: Vec::new()
                };

                // Get map values
                while lines_iter.peek().is_some_and(|l| l.len() > 1) {
                    let line = lines_iter.next().unwrap();
                    match line.split_whitespace().map(|info: &str| return info.parse::<u64>().unwrap()).collect::<Vec<u64>>()[..] {
                        [start_dst, start_src, range, ..] => {
                            let new_range_map = RangeMap { start_src: start_src, start_dst: start_dst, range: range };
                            match almanac_map.map.binary_search_by_key(&start_src, |range_map: &RangeMap| range_map.start_src) {
                                Ok(_) => {},
                                Err(pos) => {
                                    almanac_map.map.insert(pos, new_range_map);
                                },
                            }
                        },
                        _ => {}
                    }
                }
                lines_iter.next();
                almanac.maps.push(almanac_map);
            }
        }
        return almanac;
    }
}

fn main() {
    println!("Hello, world!");
    let almanac: Almanac = Almanac::load_almanac_from_file(FILE_NAME);
    let min_location: u64 = almanac.get_seeds_to_locations().pop().unwrap().0;
    println!("Mininum seed location: {}", min_location);
    let min_range_location: u64 = almanac.get_minimum_location_from_seed_ranges();
    println!("Mininum range seed location: {}", min_range_location);
}

#[cfg(test)]
mod tests {
    use std::{collections::BinaryHeap, cmp::Reverse};

    use crate::Almanac;

    #[test]
    fn test_seed_to_location() {
        let almanac: Almanac = Almanac::load_almanac_from_file("test_input.txt");
        let mut seeds_to_locations = almanac.get_seeds_to_locations();

        let min = seeds_to_locations.peek().unwrap().0;

        let mut comparison: BinaryHeap<Reverse<u64>> = BinaryHeap::from(
                [
                    Reverse(82),
                    Reverse(43),
                    Reverse(86),
                    Reverse(35)
                ]
            );
        while !seeds_to_locations.is_empty() {
            assert_eq!((&seeds_to_locations.pop().unwrap().0), (&comparison.pop().unwrap().0));
        }
        assert_eq!(min, 35_u64);
    }

    #[test]
    fn test_seed_ranges_to_location() {
        let almanac: Almanac = Almanac::load_almanac_from_file("test_input.txt");
        let seeds_to_locations = almanac.get_minimum_location_from_seed_ranges();

        assert_eq!(seeds_to_locations, 46);
    }
}