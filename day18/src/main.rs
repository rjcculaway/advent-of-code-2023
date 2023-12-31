//--------------------------------------------------------------------------------
// Day 18: Lavaduct Lagoon
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

const FILE_NAME: &'static str = "input.txt";

struct DigStep {
    direction: char,
    steps: u64,
}

struct DigPlan {
    plan: Vec<DigStep>
}

impl DigPlan {
    pub fn load_from_file(file_name: &str) -> Self {
        let mut plan: Vec<DigStep> = vec![];

        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            for line in file_contents.lines() {
                let plan_components = line.split_whitespace();
                match plan_components.collect::<Vec<&str>>()[..3] {
                    [dir_str, steps_str, _] => {
                        let direction: char = dir_str.chars().next().unwrap();
                        let steps: u64 = steps_str.parse().unwrap();

                        plan.push(DigStep { direction, steps });
                    }
                    _ => unreachable!()
                }
            }
        }
        
        DigPlan { plan }
    }

    pub fn load_from_file_corrected(file_name: &str) -> Self {
        let mut plan: Vec<DigStep> = vec![];

        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            for line in file_contents.lines() {
                let plan_components = line.split_whitespace();
                match plan_components.collect::<Vec<&str>>()[..3] {
                    [_, _, color_str] => {
                        let correct_instruction_iter = color_str.chars().skip(2);
                        let steps_str = correct_instruction_iter.clone().take(5).collect::<String>();
                        // println!("{}", steps_str);
                        let steps = u64::from_str_radix(&steps_str, 16).unwrap();
                        let direction_id: char = correct_instruction_iter.clone().skip(5).next().unwrap();
                        // println!("{}", direction_id);
                        let direction: char = match direction_id.to_digit(10).unwrap() {
                            0 => 'R',
                            1 => 'D',
                            2 => 'L',
                            3 => 'U',
                            _ => unreachable!()
                        };

                        plan.push(DigStep { direction, steps });
                    }
                    _ => unreachable!()
                }
            }
        }
        
        DigPlan { plan }
    }

    // Returns vertices in counter-clockwise order and the number of boundary points
    fn get_vertices(&self) -> (Vec<(i64, i64)>, u64) {
        let mut vertices: Vec<(i64, i64)> = vec![(0, 0)];

        let mut current_pos: (i64, i64) = (0, 0);
        let mut boundary_points: u64 = 0;

        for dig_step in &self.plan {
            let DigStep { direction, steps} = dig_step;
            match direction {
                'U' => {
                    current_pos.1 -= *steps as i64;
                },
                'D' => {
                    current_pos.1 += *steps as i64;
                },
                'L' => {
                    current_pos.0 -= *steps as i64;
                },
                'R' => {
                    current_pos.0 += *steps as i64;
                },
                _ => unreachable!()
            }
            vertices.insert(0, current_pos);
            boundary_points += *steps as u64;
            // vertices.push(current_pos);
        }
        // println!("{:?}", (&vertices, &boundary_points));
        (vertices, boundary_points)
    }
}

fn shoelace(vertices: &Vec<(i64, i64)>) -> u64 {
    let mut area: i64 = 0;
    for pair in vertices.windows(2) {
        match pair[..2] {
            [v1, v2] => {
                let v1_x: i64 = v1.0.try_into().unwrap();
                let v1_y: i64 = v1.1.try_into().unwrap();
                let v2_x: i64 = v2.0.try_into().unwrap();
                let v2_y: i64 = v2.1.try_into().unwrap();
                area += (v2_x - v1_x) * (v2_y + v1_y);
            }
            _ => unreachable!()
        }
    }
    (i64::abs(area / 2) + 1).try_into().unwrap()
}

fn picks(vertex_info: &(Vec<(i64, i64)>, u64)) -> i64 {
    let (vertices, boundary_points) = vertex_info;
    let interior: i64 = shoelace(vertices).try_into().unwrap_or(0);
    interior + i64::try_from(boundary_points / 2).unwrap()
}

fn main() {
    let dig_plan = DigPlan::load_from_file(FILE_NAME);
    let vertices = dig_plan.get_vertices();
    println!("{:?}", picks(&vertices));
    
    let dig_plan_correct = DigPlan::load_from_file_corrected(FILE_NAME);
    let vertices = dig_plan_correct.get_vertices();
    println!("{:?}", picks(&vertices));
}

#[cfg(test)]
mod test {
    use crate::{DigPlan, picks};

    #[test]
    pub fn get_area() {
        let dig_plan = DigPlan::load_from_file("test_input.txt");
        assert_eq!(62, picks(&dig_plan.get_vertices()));
    }

    #[test]
    pub fn get_area_corrected() {
        let dig_plan = DigPlan::load_from_file_corrected("test_input.txt");
        assert_eq!(952408144115, picks(&dig_plan.get_vertices()));
    }
}