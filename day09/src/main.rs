//--------------------------------------------------------------------------------
// Day 09: Mirage Maintenance
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::fs;

const FILE_NAME: &'static str = "input.txt";

#[derive(Debug)]
struct Extrapolator {
    values: Vec<i64>, 
}

impl Extrapolator {
    pub fn load_from_file(file_name: &str) -> Vec<Extrapolator> {
        let mut extrapolators = vec![];

        let Ok(file_contents) = fs::read_to_string(file_name) else {
            return extrapolators;
        };

        for line in file_contents.lines() {
            extrapolators.push(Extrapolator { values: line.split_ascii_whitespace().map(|number| number.parse().unwrap_or(0)).collect() } );
        }

        return extrapolators;
    }

    fn compute_deltas(&self) -> Vec<Vec<i64>> {
        let mut delta_levels: Vec<Vec<i64>> = vec![self.values.clone()];
        
        loop {
            let Some(last_level) = delta_levels.last() else {
                break;
            };
            let Some(first_element) = last_level.first() else {
                break;
            };
            if !last_level.iter().skip(1).any(|elem| elem != first_element) {
                break;
            }
            delta_levels.push(last_level.windows(2).map(|window| {
                match window {
                    [left, right] => {
                        return right - left;
                    },
                    _ => 0
                }
            }).collect());
        }
        
        return delta_levels;
    }

    pub fn extrapolate_next(&self) -> i64 {
        let deltas: Vec<Vec<i64>> = self.compute_deltas();
        let next: Option<i64> = deltas.iter().map(|deltas| deltas.last().copied().unwrap_or(0)).reduce(|prev_delta, curr_delta| prev_delta + curr_delta);
        match next {
            None => 0,
            Some(result) => result 
        }
    }

    pub fn extrapolate_previous(&self) -> i64 {
        let deltas: Vec<Vec<i64>> = self.compute_deltas();
        let next: Option<i64> = deltas.iter()
                                    .map(|deltas| deltas.first().copied().unwrap_or(0))
                                    .enumerate()
                                    .map(|(i, delta)| match i % 2 { 1 => -delta, _ => delta })
                                    .reduce(|prev, next| prev + next);
        match next {
            None => 0,
            Some(result) => result 
        }
    }

}


fn main() {
    println!("Hello, world!");
    let extrapolators = Extrapolator::load_from_file(FILE_NAME);
    println!("Sum of next extrapolations: {}", extrapolators.iter().map(|extrapolator| return extrapolator.extrapolate_next()).reduce(|prev: i64, next: i64| prev + next).unwrap_or(0));
    println!("Sum of previous extrapolations: {}", extrapolators.iter().map(|extrapolator| return extrapolator.extrapolate_previous()).reduce(|prev: i64, next: i64| prev + next).unwrap_or(0));
}

#[cfg(test)]
mod test {
    use std::iter::zip;
    use crate::Extrapolator;

    #[test]
    fn test_extrapolate_next() {
        let file_name: &'static str = "test_input.txt";
        let correct_answers: [i64; 3] = [18, 28, 68];
        let extrapolators = Extrapolator::load_from_file(file_name);
        for (extrapolator, correct_answer) in zip(extrapolators, correct_answers) {
            assert_eq!(extrapolator.extrapolate_next(), correct_answer);
        }
    }

    #[test]
    fn test_extrapolate_previous() {
        let file_name: &'static str = "test_input.txt";
        let correct_answer: i64 = 5;
        let extrapolators = Extrapolator::load_from_file(file_name);
        let Some(extrapolator) = extrapolators.last() else {
            panic!();
        };

        assert_eq!(extrapolator.extrapolate_previous(), correct_answer);
    }
}
