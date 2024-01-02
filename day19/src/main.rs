//--------------------------------------------------------------------------------
// Day 19: Aplenty
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::{collections::{HashMap, VecDeque}, fs::read_to_string};

const FILE_NAME: &'static str = "input.txt";

#[derive(Debug)]
enum RuleResult {
    NextWorkflow(String),
    NotMatching,
    Accept,
    Reject
}

#[derive(Debug, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

#[derive(Debug)]
struct Sorter {
    workflows: HashMap<String, Vec<String>>,
    parts: Vec<Part>
}

impl Sorter {
    pub fn load_from_file(file_name: &str) -> Self {
        let mut workflows: HashMap<String, Vec<String>> = HashMap::new();
        let mut parts: Vec<Part> = vec![];
    
        if let Ok(file_contents) = read_to_string(file_name) {
            let lines = file_contents.lines();
            for line in lines.clone().take_while(|line| line.len() > 0) {
                match line.split("{").collect::<Vec<&str>>()[..2] {
                    [workflow_name, rest] => {
                        let mut rest_chars = rest.chars();
                        rest_chars.next_back(); // Remove end curly braces
                        let rest_removed_curly = rest_chars.as_str();
                        let workflow_conditions = rest_removed_curly.split(",").map(|condition| condition.to_string()).collect::<Vec<String>>();
                        workflows.insert(workflow_name.to_string(), workflow_conditions);
                    }
                    _ => unreachable!()
                }
            }
            
            let mut parts_lines = lines.clone().skip_while(|line| line.len() > 0);
            parts_lines.next();
            for line in parts_lines {
                let mut line_chars = line.chars();
                line_chars.next();
                line_chars.next_back();
                let ratings_str = line_chars.as_str();
                match ratings_str.split(",").collect::<Vec<&str>>()[..4] {
                    [x_rating, m_rating, a_rating, s_rating] => {
                        let x: u32 = x_rating.split("=").last().unwrap().parse::<u32>().unwrap();
                        let m: u32 = m_rating.split("=").last().unwrap().parse::<u32>().unwrap();
                        let a: u32 = a_rating.split("=").last().unwrap().parse::<u32>().unwrap();
                        let s: u32 = s_rating.split("=").last().unwrap().parse::<u32>().unwrap();
                        parts.push(Part { x, m, a, s });
                    },
                    _ => unreachable!()
                }
            }
        }
    
        Sorter { workflows, parts }
    }

    fn process_rule(rule: &String, part: &Part) -> RuleResult {
        match rule.split(":").collect::<Vec<&str>>()[..] {
            [condition, result_if_true] => {

                let (rating, rest) = condition.split_at(1);
                let (relation, value_str) = rest.split_at(1);
                let value: u32 = value_str.parse().unwrap();

                let value_to_test = match rating {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => unreachable!()
                };

                let is_match: bool = match relation {
                    ">" => value_to_test > value,
                    "<" => value_to_test < value,
                    _ => unreachable!()
                };

                if !is_match {
                    return RuleResult::NotMatching;
                }

                match result_if_true {
                    "A" => { RuleResult::Accept }
                    "R" => { RuleResult::Reject }
                    _ => { RuleResult::NextWorkflow(result_if_true.to_string()) }
                }
            },
            [unconditonal_result] => {
                match unconditonal_result {
                    "A" => { RuleResult::Accept }
                    "R" => { RuleResult::Reject }
                    _ => { RuleResult::NextWorkflow(unconditonal_result.to_string()) }
                }
            }
            _ => unreachable!()
        }
    }

    pub fn process_workflow(&self, workflow_name: &String, part: &Part) -> RuleResult {
        let workflow = self.workflows.get(workflow_name).unwrap();
        let mut result = RuleResult::NotMatching;
        for rule in workflow {
            match Self::process_rule(rule, part) {
                RuleResult::NotMatching => {},
                next_result @ _ => {
                    result = next_result;
                    break;
                }
            }
        }

        result
    }

    pub fn process_parts(&self) -> u32 {
        let mut accepted: Vec<Part> = vec![];
        for part in &self.parts {
            let mut current_result: RuleResult = RuleResult::NextWorkflow("in".to_string());

            // println!("{:?}", part);
            loop {
                // println!("{:?}", current_result);
                match current_result {
                    RuleResult::Accept => {
                        accepted.push(part.clone());
                        break;
                    },
                    RuleResult::NextWorkflow(next @ _) => {
                        current_result = self.process_workflow(&next, part);
                    },
                    RuleResult::Reject| RuleResult::NotMatching => {
                        break;
                    },
                }
            }
            
        }
        
        accepted.iter().map(|part| part.x + part.m + part.a + part.s).sum()
    }

    // In part 2, the problem becomes similar to the seed mapping day.
    pub fn count_accepted_permutations(&self, min: u32, max: u32) -> u64 {
        let mut accepted: Vec<((u32, u32), (u32, u32), (u32, u32), (u32, u32))> = vec![];
        // let rejected: Vec<(u32, u32)> = vec![()];

        let mut to_process: VecDeque<((u32, u32), (u32, u32), (u32, u32), (u32, u32), String)> = VecDeque::from(
            [
                ((min, max), (min, max), (min, max), (min, max), "in".to_string()), 
            ]
        );
        while let Some(current) = to_process.pop_front() {
            let (mut current_x, mut current_m, mut current_a, mut current_s, workflow_name) = current;
            let workflow = self.workflows.get(&workflow_name).unwrap();

            for rule in workflow {
                if [current_x, current_m, current_a, current_s].iter().any(|(start, end)| end < start) {
                    break;
                }
                match rule.split(":").collect::<Vec<&str>>()[..] {
                    [condition, result_if_true] => {

                        let (rating, rest) = condition.split_at(1);
                        let (relation, value_str) = rest.split_at(1);
                        let value: u32 = value_str.parse().unwrap();

                        let (start_rating_to_split, end_rating_to_split): (u32, u32) = match rating {
                            "x" => current_x,
                            "m" => current_m,
                            "a" => current_a,
                            "s" => current_s,
                            _ => unreachable!()
                        };

                        let mut passing_range: Option<(u32, u32)> = None;
                        let nonpassing_range: (u32, u32);

                        match relation {
                            ">" => {
                                let next_passing_start: u32 = (value + 1).max(start_rating_to_split);
                                let next_passing_end: u32 = end_rating_to_split;
                                if next_passing_end >= next_passing_start {
                                    passing_range = Some((next_passing_start, next_passing_end));
                                }

                                let next_nonpassing_start = start_rating_to_split;
                                let next_nonpassing_end = value.min(end_rating_to_split);

                                nonpassing_range = (next_nonpassing_start, next_nonpassing_end);

                                println!("using rule {}, split to: ({}, {}), ({}, {}) for {} at {}", rule, start_rating_to_split, end_rating_to_split, next_passing_start, next_passing_end, rating, workflow_name);
                            },
                            "<" => {
                                let next_passing_start: u32 = start_rating_to_split;
                                let next_passing_end: u32 = (value - 1).min(end_rating_to_split);
                                if next_passing_end >= next_passing_start {
                                    passing_range = Some((next_passing_start, next_passing_end));
                                }

                                let next_nonpassing_start = value.max(start_rating_to_split);
                                let next_nonpassing_end = end_rating_to_split;

                                nonpassing_range = (next_nonpassing_start, next_nonpassing_end);

                                println!("using rule {}, split to: ({}, {}), ({}, {}) for {} at {}", rule, start_rating_to_split, end_rating_to_split, next_passing_start, next_passing_end, rating, workflow_name);
                            },
                            _ => unreachable!()
                        };

                        if let Some(passing_range) = passing_range {
                            let (next_x, next_m, next_a, next_s) = match rating {
                                "x" => (passing_range, current_m, current_a, current_s),
                                "m" => (current_x, passing_range, current_a, current_s),
                                "a" => (current_x, current_m, passing_range, current_s),
                                "s" => (current_x, current_m, current_a, passing_range),
                                _ => unreachable!()
                            };
                            match result_if_true {
                                "A" => { accepted.push((next_x, next_m, next_a, next_s)) },
                                "R" => { println!("rejected at {:?}, {:?}, {:?}, {:?}", next_x, next_m, next_a, next_s) },
                                next_workflow @ _ => { to_process.push_back((next_x, next_m, next_a, next_s, String::from(next_workflow))) }
                            }
                        }
                        let (leftover_x, leftover_m, leftover_a, leftover_s) = match rating {
                            "x" => (nonpassing_range, current_m, current_a, current_s),
                            "m" => (current_x, nonpassing_range, current_a, current_s),
                            "a" => (current_x, current_m, nonpassing_range, current_s),
                            "s" => (current_x, current_m, current_a, nonpassing_range),
                            _ => unreachable!()
                        };

                        current_x = leftover_x;
                        current_m = leftover_m;
                        current_a = leftover_a;
                        current_s = leftover_s;
                    },
                    [unconditonal_result] => {
                        println!("{:?}, {:?}, {:?}, {:?} for {}", current_x, current_m, current_a, current_s, workflow_name);
                        match unconditonal_result {
                            "A" => { accepted.push((current_x, current_m, current_a, current_s)) },
                            "R" => { println!("rejected at {:?}, {:?}, {:?}, {:?} for {}", current_x, current_m, current_a, current_s, workflow_name) },
                            next_workflow @ _ => { to_process.push_back((current_x, current_m, current_a, current_s, String::from(next_workflow))) }
                        }
                    }
                    _ => unreachable!()
                }
            }
        }
        println!("{:#?}", accepted);
        accepted.iter()
        .map(|(x, m, a, s)| {
            [x, m, a, s].iter()
            .map(|(start, end)| {
                (end - start + 1) as u64
            })
            .product::<u64>()
        })
        .sum()

    }
}

fn main() {
    let sorter: Sorter = Sorter::load_from_file(FILE_NAME);
    println!("{:?}", sorter);
    println!("{:?}", sorter.process_parts());
    println!("{:?}", sorter.count_accepted_permutations(1, 4000));
}

#[cfg(test)]
mod test {
    use crate::Sorter;

    #[test]
    fn test_process_parts() {
        let sorter: Sorter = Sorter::load_from_file("test_input.txt");
        assert_eq!(sorter.process_parts(), 19114);
    }

    #[test]
    fn test_count_accepted_permutations() {
        let sorter: Sorter = Sorter::load_from_file("test_input.txt");
        assert_eq!(sorter.count_accepted_permutations(1, 4000), 167409079868000);
    }
}