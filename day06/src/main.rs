//--------------------------------------------------------------------------------
// Day 06: Wait For It
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::fs;
use std::iter;
use std::ops::RangeInclusive;

const FILE_NAME: &str = "input.txt";

#[derive(Debug, PartialEq)]
struct RaceData {
    time: u64,
    distance: u64
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
    let negative_b: f64 = -b;
    let square_root: f64 = f64::sqrt(b * b - 4_f64 * a * c);
    let a_2: f64 = 2_f64 * a;

    return ((negative_b + square_root) / a_2, (negative_b - square_root) / a_2);
}

impl RaceData {
    pub fn get_winning_charge_times(&self) -> RangeInclusive<u64> {
        // To get the winning charge times, we need to solve a system of equations.
        // Let:
        // t_i              be the time the toy car is charged.
        // t_max (known)    be the race data's time. This is considered the maximum time to beat the record.
        // d_i              the distance traveled using the charge time.
        // d_min (known)    be the the race data's distance. This is considered the minimum distance to beat the record.

        // Using the following notation, we can construct the following system:
        // (1) t_i < t_max                  The time taken to charge must strictly be less then the maximum time.             
        // (2) d_i = t_i(t_max - t_i)       The distance travelled is given as a function of the time taken to charge.
        // (3) d_min < d_i                  The distance travelled should exceed the minimum distance.

        // This can be simplified to the following QUADRATIC equation.
        // d_i = -(t_i ^ 2) + t_max * t_i
        //      or
        // y = -x^2 + t_max * x
        // 
        // Note that d_i must be greater than d_min as per Eq. 3, so we include it in the final equation as well.
        // y = -x^2 + t_max * x - d_min

        let solution = quadratic_formula(-1_f64, self.time as f64, -(self.distance as f64));
        return ((solution.0 + 1.0).floor() as u64)..=((solution.1 - 1.0).ceil() as u64);
    }

    pub fn load_from_file(file_name: &str) -> Vec<RaceData> {
        let mut races = Vec::new();

        if let Ok(file_contents) = fs::read_to_string(file_name) {
            match file_contents.lines().collect::<Vec<&str>>()[..] {
                [times_line, distances_line] => {
                    let times = times_line.split_whitespace().skip(1).map(|time_str: &str| time_str.parse::<u64>().unwrap_or(0));
                    let distances = distances_line.split_whitespace().skip(1).map(|distance_str: &str| distance_str.parse::<u64>().unwrap_or(0));
                    for (time, distance) in iter::zip(times, distances) {
                        races.push(RaceData { time, distance });
                    }
                },
                _ => {}
            }
        }

        return races;
    } 

    pub fn load_from_file_ignore_spaces(file_name: &str) -> RaceData {
        if let Ok(file_contents) = fs::read_to_string(file_name) {
            match file_contents.lines().collect::<Vec<&str>>()[..] {
                [times_line, distances_line] => {
                    let time = times_line.split(": ").nth(1).unwrap_or("").split_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap_or(0);
                    let distance = distances_line.split(": ").nth(1).unwrap_or("").split_whitespace().collect::<Vec<&str>>().join("").parse::<u64>().unwrap_or(0);
                    RaceData { time, distance }
                },
                _ => {
                    RaceData { time: 0, distance: 0 }
                }
            }
        } else {
            RaceData { time: 0, distance: 0 }
        }
    } 
}

fn count_possible_ways_to_win(race_data: &Vec<RaceData>) -> u64 {
    let mut error_margins: Vec<u64> = Vec::new();
    for race_data in race_data {
        let winning_charge_times: RangeInclusive<u64> = race_data.get_winning_charge_times();
        let range_of_winning = winning_charge_times.size_hint().0.try_into().unwrap_or(0);
        println!("range of winning: {range_of_winning} (from {:#?})", winning_charge_times);
        error_margins.push(range_of_winning);
    }

    return error_margins.into_iter().reduce(|prev, curr| prev * curr).unwrap_or(0);
}

fn main() {
    println!("Hello, world!");
    let race_data: Vec<RaceData> = RaceData::load_from_file(FILE_NAME);
    println!("{:?}", race_data);
    println!("{:?}", count_possible_ways_to_win(&race_data));
    let race_data_ignored_space: RaceData = RaceData::load_from_file_ignore_spaces(FILE_NAME);
    println!("{:?}", race_data);
    println!("{:?}", count_possible_ways_to_win(&vec![race_data_ignored_space]));
}

#[cfg(test)]
mod test {
    use std::ops::RangeInclusive;
    use std::iter::zip;

    use crate::RaceData;

    #[test]
    fn test_load_from_file () {
        assert_eq!(RaceData::load_from_file("test_input.txt"), vec![
            RaceData { time: 7, distance: 9 },
            RaceData { time: 15, distance: 40 },
            RaceData { time: 30, distance: 200 }
        ]);
    }

    #[test]
    fn test_load_from_file_ignore_spaces () {
        assert_eq!(RaceData::load_from_file_ignore_spaces("test_input.txt"),
            RaceData { time: 71530, distance: 940200 },
        );
    }

    #[test]
    fn test_get_winning_charge_times() {
        let races: Vec<RaceData> = RaceData::load_from_file("test_input.txt");
        const CORRECT_CHARGE_TIMES: [RangeInclusive<u64>; 3] = [
            2..=5,
            4..=11,
            11..=19,
        ];
        for (race, result) in zip(races, CORRECT_CHARGE_TIMES) {
            assert_eq!(race.get_winning_charge_times(), result);
            break;
        }
    }

    #[test]
    fn test_count_possible_ways_to_win() {
        let races: Vec<RaceData> = RaceData::load_from_file("test_input.txt");
        assert_eq!(crate::count_possible_ways_to_win(&races), 288);
    }

    #[test]
    fn test_count_possible_ways_to_win_ignore_spaces() {
        let races: Vec<RaceData> = vec![RaceData::load_from_file_ignore_spaces("test_input.txt")];
        assert_eq!(crate::count_possible_ways_to_win(&races), 71503);
    }
}
