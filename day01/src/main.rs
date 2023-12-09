//--------------------------------------------------------------------------------
// Day 01: Trebuchet?!
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

const FILE_NAME: &str = "input.txt";

mod part_one {
    fn get_calibration_value(line: &str) -> u32 { 
        let mut combined: std::string::String = String::new();
    
        for c in line.chars() {
            if c.is_numeric() {
                combined.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                combined.push(c);
                break;
            }
        }
    
        if let Ok(number) = combined.parse::<u32>() {
            return number;
        }
    
        return 0;
    }
    
    pub fn get_cumulative_calibration_value(file_name: &str) -> u32 {
        let mut calibration_value_sum = 0;
        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            for line in file_contents.lines() {
                calibration_value_sum += get_calibration_value(line);
            }
            
        }
        return calibration_value_sum;
    }
}

mod part_two {

    fn word_to_digit(word: &str) -> std::option::Option<u32> {
        match word {
            "one" => return Some(1),
            "two" => return Some(2),
            "three" => return Some(3),
            "four" => return Some(4),
            "five" => return Some(5),
            "six" => return Some(6),
            "seven" => return Some(7),
            "eight" => return Some(8),
            "nine" => return Some(9),
            "zero" => return Some(0),
            _ => return None
        }
    }

    fn get_calibration_value(line: &str) -> u32 { 
        let word_digits: [&str; 10] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero"
        ];

        let mut first_digit_index: std::option::Option<usize> = None;
        let mut first_digit: std::option::Option<u32> = None;

        let mut second_digit_index: std::option::Option<usize> = None;
        let mut second_digit: std::option::Option<u32> = None;

        for word_digit in word_digits.iter() {
            if let Some(word_digit_idx) = line.find(word_digit) {
                if first_digit_index.is_none() || word_digit_idx < first_digit_index.unwrap() {
                    first_digit_index = Some(word_digit_idx);
                    first_digit = word_to_digit(word_digit);
                }
            }
            if let Some(word_digit_idx) = line.rfind(word_digit) {
                if second_digit_index.is_none() || word_digit_idx > second_digit_index.unwrap() {
                    second_digit_index = Some(word_digit_idx);
                    second_digit = word_to_digit(word_digit);
                }
            }
        }

        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() && (first_digit.is_none() || first_digit_index.unwrap() > i)  {
                // first_digit_index = Some(i);
                first_digit = Some(c.to_digit(10).unwrap());
                break;
            }
        }
        for (i, c) in line.chars().rev().enumerate() {
            if c.is_numeric() && (second_digit.is_none() || second_digit_index.unwrap() < line.len() - 1 - i)  {
                // second_digit_index = Some(i);
                second_digit = Some(c.to_digit(10).unwrap());
                break;
            }
        }



        return first_digit.unwrap_or(0) * 10 + second_digit.unwrap_or(0);
    }

    pub fn get_cumulative_calibration_value(file_name: &str) -> u32 {
        let mut calibration_value_sum = 0;
        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            for line in file_contents.lines() {
                let calibration_value: u32 = get_calibration_value(line);
                calibration_value_sum += calibration_value;
            }
            
        }
        return calibration_value_sum;
    }
}


fn main() {
    println!("{}", part_one::get_cumulative_calibration_value(FILE_NAME));
    println!("{}", part_two::get_cumulative_calibration_value(FILE_NAME));
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

    #[test]
    fn test_get_calibration_value() {
        assert_eq!(crate::part_one::get_cumulative_calibration_value("test_input.txt"), 142);
        assert_eq!(crate::part_two::get_cumulative_calibration_value("test_input2.txt"), 281);
    }
}