//--------------------------------------------------------------------------------
// Day 03: Gear Ratios
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::collections::HashMap;
use std::option::Option;
use std::fs;
use std::str;
use std::ops::Range;

struct GearRatio {
    involved_engine_parts: u32,
    value: u32
}

const FILE_NAME: &str = "input.txt";

fn is_engine_part(row: usize, number_sequence: Range<usize>, schematic: &Vec<Vec<u8>>) -> (bool, u32) {
    let rows = schematic.len();
    let columns = schematic[0].len();

    let mut engine_part_value: u32 = 0;
    let mut is_part: bool = false;
    let num_of_digits: u32 = number_sequence.len() as u32;

    for (k, i) in number_sequence.enumerate() {
        let digit_value: u32 = (schematic[row][i] as char).to_digit(10).unwrap_or(0);
        
        // Search all adjacent spaces for a symbol
        if !is_part {   // Skip checking if we already found out that it is a part
            for y in row.checked_sub(1).unwrap_or(row)..(row + 2).min(rows) {
                for x in i.checked_sub(1).unwrap_or(i)..(i + 2).min(columns) {
                    let schematic_cell: char = schematic[y][x].clone() as char;
                    let is_adjacent_a_symbol = schematic_cell != '.' && !schematic_cell.is_numeric();

                    if is_adjacent_a_symbol {
                        is_part = true;
                        break;
                    }
                }
            }
        }
        engine_part_value += digit_value * 10_u32.pow(num_of_digits - (k as u32) - 1);
    }
    
    // println!("Line {} {} {} an engine part.", row + 1, engine_part_value, if is_part { "is" } else { "is not" });
    return (is_part, engine_part_value);
}

fn load_engine_schematic(file_name: &str) -> Option<Vec<Vec<u8>>> {
    if let Ok(file_contents) = fs::read_to_string(file_name) {
        let engine_schematic: Vec<Vec<u8>> = file_contents.lines().map(|line| std::string::String::from(line).into_bytes()).collect();
        return Some(engine_schematic);
    }
    return None;
}

fn sum_engine_parts(file_name: &str) -> u32 {
    let mut engine_part_sum: u32 = 0;
    if let Some(engine_schematic) = load_engine_schematic(file_name) {
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        
        for i in 0..engine_schematic.len() {
            for j in 0..engine_schematic[i].len() {
                // print!("{}", engine_schematic[i][j] as char);
                let schematic_cell: char = engine_schematic[i][j] as char;
                
                if char::is_numeric(schematic_cell) {
                    end = Some(j);
                    if start.is_none() {
                        start = Some(j);
                    }
                }

                if (!schematic_cell.is_numeric() || j == engine_schematic[i].len() - 1) && start.is_some() {
                    let (is_part, value) = is_engine_part(i, start.unwrap()..end.unwrap() + 1, &engine_schematic);
                    
                    if is_part {
                        engine_part_sum += value;
                    }

                    start = None;
                    end = None;
                }
            }
            // print!("\n");
        }
        // println!("{}", engine_part_sum);
    }
    return engine_part_sum;
}

fn is_gear_part(row: usize, number_sequence: Range<usize>, schematic: &Vec<Vec<u8>>) -> Option<(u32, usize, usize)> {
    let rows = schematic.len();
    let columns = schematic[0].len();

    let mut engine_part_value: u32 = 0;
    let mut is_part: bool = false;
    let mut gear_row: usize = 0;
    let mut gear_column: usize = 0;
    let num_of_digits: u32 = number_sequence.len() as u32;

    for (k, i) in number_sequence.enumerate() {
        let digit_value: u32 = (schematic[row][i] as char).to_digit(10).unwrap_or(0);
        
        // Search all adjacent spaces for a symbol
        if !is_part {   // Skip checking if we already found out that it is a part
            for y in row.checked_sub(1).unwrap_or(row)..(row + 2).min(rows) {
                for x in i.checked_sub(1).unwrap_or(i)..(i + 2).min(columns) {
                    let schematic_cell: char = schematic[y][x].clone() as char;
                    let is_adjacent_a_gear = schematic_cell == '*';

                    if is_adjacent_a_gear {
                        is_part = true;
                        gear_row = y;
                        gear_column = x;
                        break;
                    }
                }
            }
        }
        engine_part_value += digit_value * 10_u32.pow(num_of_digits - (k as u32) - 1);
    }
    
    // println!("Line {} {} {} a gear part.", row + 1, engine_part_value, if is_part { "is" } else { "is not" });
    if is_part {
        return Some((engine_part_value, gear_row, gear_column));
    }
    return None;
}

fn get_gear_ratios(file_name: &str) -> HashMap<String, GearRatio> {
    let mut gear_ratios: HashMap<String, GearRatio> = HashMap::new();
    if let Some(engine_schematic) = load_engine_schematic(file_name) {
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        
        for i in 0..engine_schematic.len() {
            for j in 0..engine_schematic[i].len() {
                // print!("{}", engine_schematic[i][j] as char);
                let schematic_cell: char = engine_schematic[i][j] as char;
                
                if char::is_numeric(schematic_cell) {
                    end = Some(j);
                    if start.is_none() {
                        start = Some(j);
                    }
                }

                if (!schematic_cell.is_numeric() || j == engine_schematic[i].len() - 1) && start.is_some() {
                    if let Some((value, gear_row, gear_column)) = is_gear_part(i, start.unwrap()..end.unwrap() + 1, &engine_schematic) {
                        let key: String = format!("{}_{}", gear_row, gear_column);
                        if gear_ratios.contains_key(&key) {
                            let gear_ratio: &mut GearRatio = gear_ratios.get_mut(&key).unwrap();
                            gear_ratio.involved_engine_parts += 1;
                            gear_ratio.value *= value;
                        } else {
                            gear_ratios.insert(key, GearRatio { involved_engine_parts: 1, value: value });
                        }
                    }
                    start = None;
                    end = None;
                }
            }
            // print!("\n");
        }
    }
    return gear_ratios;
}

fn sum_gear_ratios(gear_ratios: HashMap<String, GearRatio>) -> u32 {
    let mut sum = 0;
    for (_, v) in gear_ratios.iter() {
        if v.involved_engine_parts > 1 {
            sum += v.value;
        }
    }
    return sum;
}

fn main() {
    sum_engine_parts(FILE_NAME);
    println!("{}", sum_gear_ratios(get_gear_ratios(FILE_NAME)));
}

#[cfg(test)]
mod test {
    use crate::{sum_engine_parts, sum_gear_ratios, get_gear_ratios};

    #[test]
    fn test_sum_engine_parts() {
        assert_eq!(sum_engine_parts("test_input.txt"), 4361);
        assert_eq!(sum_engine_parts("test_input2.txt"), 0);
    }
    #[test]
    fn test_sum_gear_ratios() {
        assert_eq!(sum_gear_ratios(get_gear_ratios("test_input.txt")), 467835);
        assert_eq!(sum_gear_ratios(get_gear_ratios("test_input2.txt")), 0);
    }
}