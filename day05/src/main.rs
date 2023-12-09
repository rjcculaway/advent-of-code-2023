//--------------------------------------------------------------------------------
// Day 05: If You Give A Seed A Fertilizer
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::fs;
use std::ops::{Range, Sub};

const FILE_NAME: &str = "input.txt";

#[derive(Clone)]
struct RangeMap {
    start_src: u64,
    start_dst: u64,
    range: u64
}

impl RangeMap {
    fn get_range(&self, start: u64, range: u64) -> Range<u64> {
        return start..start+range;
    }
    pub fn compose_mapping(&self, from: &RangeMap) -> Option<RangeMap> {
        let mut intersection: Option<RangeMap> = None;

        let from_domain = from.get_range(from.start_src, from.start_src + from.range);
        let current_domain = self.get_range(self.start_src, self.start_src + self.range);
        
        let current_range = self.get_range(self.start_dst, self.start_dst + self.range);
        let from_range = from.get_range(from.start_dst, from.start_dst + from.range);

        if from_domain.start <= current_domain.end && current_domain.end >= from_domain.start {
            let new_start = from_domain.start.max(current_domain.start);
            let new_end = from_domain.end.min(current_domain.end);
            intersection = Some(RangeMap { start_src: new_start, start_dst: from_range.start.max(current_range.start), range: (new_start..new_end).size_hint().0 as u64 })
        }

        return intersection;
    }
    pub fn convert_to_value(&self, key: u64) -> u64 {
        return self.start_dst + (key - self.start_src);
    }

    pub fn is_within_range(&self, value: &u64) -> bool {
        return (self.start_src..(self.start_src + self.range)).contains(&value);
    }
}

struct AlmanacMap {
    map_label: String,
    map: Vec<RangeMap>
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
    seed_to_location: AlmanacMap
}

impl Almanac {

    pub fn get_seeds_to_locations(&self) -> BinaryHeap<Reverse<u64>> {
        let mut seeds_to_locations: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
        
        for seed in &self.seeds {
            println!("Converting {}", seed);
            let mut current_transformation: u64 = *seed;
            
            let mut starting_map: RangeMap = RangeMap { start_src: *seed, start_dst: *seed, range: 1 };
            // Get the first mapping applicable
            for almanac_map in &self.maps[0].map {
                if almanac_map.is_within_range(seed) {
                    starting_map = almanac_map.clone();
                    break;
                }
            }

            for map in self.maps.iter().skip(1) {
                for range in &map.map {
                    let intersection = range.compose_mapping(&starting_map);
                    if intersection.is_some() {
                        starting_map = intersection.unwrap();
                    }
                }
            }

            seeds_to_locations.push(Reverse(starting_map.convert_to_value(*seed)));
        }

        return seeds_to_locations;
    }

    pub fn get_seed_ranges_to_locations(&self) -> BinaryHeap<Reverse<u64>> {
        let mut seeds_to_locations: BinaryHeap<Reverse<u64>> = BinaryHeap::new();

        for seed_chunk in self.seeds.chunks(2) {
            match seed_chunk {
                [start, range] => {
                    for seed in *start..(*start + *range) {
                        // println!("Converting {}", seed);
                        let mut current_transformation: u64 = seed;
                        for almanac_map in &self.maps {
                            // println!("Converting using {}", almanac_map.map_label);
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
                },
                _ => ()
            }
        }

        return seeds_to_locations;
    }

    pub fn load_almanac_from_file(file_name: &str) -> Almanac {

        let mut almanac: Almanac = Almanac {
            seeds: Vec::new(),
            maps: Vec::new(),
            seed_to_location: AlmanacMap { map_label: "seed-to-location".to_string(), map: Vec::new() }
        };

        if let Ok(file_contents) = fs::read_to_string(file_name) {
            let lines: Vec<&str> = file_contents.lines().collect();
            // Collect seeds
            let seeds_line = lines.get(0).unwrap();
            let seeds_space_separated = match seeds_line.split_at(7) {
                (_, value) => { value }
            };
            almanac.seeds.append(&mut seeds_space_separated.split(" ").map(|seed_str: &str| return seed_str.parse::<u64>().unwrap()).collect());
            // println!("{:?}", almanac.seeds);
            
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
                    // println!("{:?}", line);
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
        // println!("{:?}", almanac);
        return almanac;
    }
}

fn main() {
    println!("Hello, world!");
    let almanac: Almanac = Almanac::load_almanac_from_file(FILE_NAME);
    let min_location: u64 = almanac.get_seeds_to_locations().pop().unwrap().0;
    println!("Mininum seed location: {}", min_location);
    let min_range_location: u64 = almanac.get_seed_ranges_to_locations().pop().unwrap().0;
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
        let seeds_to_locations = almanac.get_seed_ranges_to_locations();

        assert_eq!(seeds_to_locations.peek().unwrap().0, 46);
    }
}