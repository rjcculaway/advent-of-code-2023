//--------------------------------------------------------------------------------
// Day 05: If You Give A Seed A Fertilizer
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::collections::HashMap;
use std::fs;
use std::iter::zip;

const FILE_NAME: &str = "input.txt";

#[derive(Debug)]
struct AlmanacMap {
    map_label: String,
    map: HashMap<u64, u64>
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    pub fn get_seeds_to_locations(&self) -> HashMap<u64, u64> {
        let mut seeds_to_locations: HashMap<u64, u64> = HashMap::new();

        
        for seed in &self.seeds {
            println!("Converting {}", seed);
            let mut current_transformation: u64 = *seed;
            for almanac_map in &self.maps {
                println!("Converting using {}", almanac_map.map_label);
                current_transformation = *almanac_map.map.get(&current_transformation).unwrap_or(&current_transformation);
            }
            seeds_to_locations.insert(*seed, current_transformation);
        }

        return seeds_to_locations;
    }

    fn parse_range(start_src: u64, start_dst: u64, range: u64, map: &mut HashMap<u64, u64>) {
        for (src, dst) in zip(start_src..start_src + range, start_dst..start_dst + range) {
            map.insert(src, dst);
        }
        
        return;
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
            // println!("{:?}", almanac.seeds);
            
            let reading_start: usize = 2;

            let mut lines_iter = lines.iter().skip(reading_start).peekable();
            while lines_iter.peek().is_some() {
                let line = lines_iter.next().unwrap();
                // println!("{:?}", line.split_whitespace().next().unwrap());
                // Extract map name
                let mut almanac_map: AlmanacMap = AlmanacMap { 
                    map_label: line.split_whitespace().next().unwrap().to_string(), 
                    map: HashMap::new()
                };

                // Get map values
                while lines_iter.peek().is_some_and(|l| l.len() > 1) {
                    let line = lines_iter.next().unwrap();
                    // println!("{:?}", line);
                    match line.split_whitespace().map(|info: &str| return info.parse::<u64>().unwrap()).collect::<Vec<u64>>()[..] {
                        [start_dst, start_src, range, ..] => Almanac::parse_range(start_src, start_dst, range, &mut almanac_map.map),
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
    let min_location: u64 = *(almanac.get_seeds_to_locations().values().min().unwrap());
    println!("Mininum location: {}", min_location);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::Almanac;

    #[test]
    fn test_seed_to_location() {
        let almanac: Almanac = Almanac::load_almanac_from_file("test_input.txt");
        let seeds_to_locations: HashMap<u64, u64> = almanac.get_seeds_to_locations();
        assert_eq!(
            seeds_to_locations,
            HashMap::from(
                [
                    (79, 82),
                    (14, 43),
                    (55, 86),
                    (13, 35)
                ]
            )
        );
        assert_eq!(*(seeds_to_locations.values().min().unwrap()), 35_u64);
    }
}