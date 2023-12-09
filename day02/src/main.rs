//--------------------------------------------------------------------------------
// Day 02: Cube Conundrum
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

const FILE_NAME: &str = "input.txt";

mod part_one {

    fn is_game_possible(game_line: &str, bag: &std::collections::HashMap<&str, u32>) -> (u32, bool) {
        let game_info: Vec<&str> = game_line.split(": ").collect();
        let game_id: u32 = game_info[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().unwrap_or(0);

        let pulls: std::str::Split<'_, &str> = game_info[1].split("; ");
        for pull in pulls {
            for cube in pull.split(", ") {
                let cube_info: Vec<&str> = cube.split_whitespace().collect();
                let cube_count: u32 = cube_info[0].parse::<u32>().unwrap_or(0);
                let cube_color: &str = cube_info[1];

                if bag.contains_key(cube_color) && bag.get(cube_color).unwrap() < &cube_count {
                    return (game_id, false);
                }
            }
        }

        return (game_id, true);
    }

    pub fn count_possible_games(file_name: &str, bag: &std::collections::HashMap<&str, u32>) -> u32 {
        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            let mut id_sum: u32 = 0;
            let game_possibilities = file_contents.lines().map(|line: &str| is_game_possible(line, &bag)).filter(|game_possibility: &(u32, bool)| game_possibility.1);
            for game_possibility in game_possibilities {
                let (id, _) = game_possibility;
                id_sum += id;
            }
            return id_sum;
        }
        return 0;
    }
}

mod part_two {
    fn get_cube_power(game_line: &str) -> u32 {
        let mut cube_power: u32 = 1; 
        let mut minimum_number_of_cubes: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();

        let game_info: Vec<&str> = game_line.split(": ").collect();

        let pulls: std::str::Split<'_, &str> = game_info[1].split("; ");
        for pull in pulls {
            for cube in pull.split(", ") {
                let cube_info: Vec<&str> = cube.split_whitespace().collect();
                let cube_count: u32 = cube_info[0].parse::<u32>().unwrap_or(0);
                let cube_color: &str = cube_info[1];

                minimum_number_of_cubes.insert(cube_color,
                     minimum_number_of_cubes.get(cube_color).unwrap_or(&cube_count).clone().max(cube_count));
            }
        }

        for (_, minimum) in minimum_number_of_cubes {
            cube_power *= minimum;
        }

        return cube_power;
    }

    pub fn sum_set_power(file_name: &str) -> u32 {
        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            let mut sum_power: u32 = 0;
            let game_powers = file_contents.lines().map(|line: &str| get_cube_power(line));
            for game_power in game_powers {
                sum_power += game_power;
            }
            return sum_power;
        }
        return 0;
    }
}

fn main() {
    let bag: std::collections::HashMap<&str, u32> = std::collections::HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    println!("{}", part_one::count_possible_games(FILE_NAME, &bag));
    println!("{}", part_two::sum_set_power(FILE_NAME));
}

#[cfg(test)]
mod test {
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn test_count_possible_games() {
        let bag: std::collections::HashMap<&str, u32> = std::collections::HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);
        const TEST_INPUT: &str = "test_input.txt";
        assert_eq!(part_one::count_possible_games(TEST_INPUT, &bag), 8);
    }

    #[test]
    fn test_sum_set_pwoer() {
        const TEST_INPUT: &str = "test_input.txt";
        assert_eq!(part_two::sum_set_power(TEST_INPUT), 2286);
    } 
}
