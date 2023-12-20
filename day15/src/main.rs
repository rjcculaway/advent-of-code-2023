//--------------------------------------------------------------------------------
// Day 15: Lens Library
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

const FILE_NAME: &'static str = "input.txt";

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u16, 
}

#[derive(Debug)]
struct Facility {
    sequence: Vec<String>,
    hash_map: Vec<Vec<Lens>>
}

impl Facility {
    pub fn load_from_file(file_name: &str) -> Self {
        let mut sequence: Vec<String> = vec![];
        let mut hash_map: Vec<Vec<Lens>> = Vec::new();

        for _ in 0..256 {
            hash_map.push(Vec::new());
        }

        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            let steps_iter = file_contents.trim().split(",").map(|string| string.to_string());
            sequence.extend(steps_iter);
        }

        Facility { sequence, hash_map }
    }

    pub fn hash(step: &String) -> i32 {
        let mut reduced: i32 = 0;

        for byte in step.as_bytes() {
            reduced += *byte as i32;
            reduced *= 17;
            reduced %= 256;
        }

        reduced
    }

    pub fn process_sequence(&mut self) {
        for step in &self.sequence {
            let instructions: Vec<&str> = step.split(&['=', '-'][..]).filter(|str| str.len() > 0).collect();
            let hash_value = Self::hash(&instructions[0].to_string());
            
            match instructions[..] {
                [label, focal_length] => {
                    if let Some(lens_box) = self.hash_map.get_mut(hash_value as usize) {
                        if let Some(to_remove) = lens_box.iter().position(|lens| lens.label == label) {
                            lens_box[to_remove].focal_length = focal_length.parse().unwrap_or(0);
                        } else {
                            lens_box.push(Lens { label: label.to_string(), focal_length: focal_length.parse().unwrap_or(0) });
                        }
                    }
                },
                [label] => {
                    if let Some(lens_box) = self.hash_map.get_mut(hash_value as usize) {
                        if let Some(to_remove) = lens_box.iter().position(|lens| lens.label == label) {
                            lens_box.remove(to_remove);
                        }
                    }
                },
                _ => unreachable!()
            }
        }
    }

    pub fn get_focusing_power(&self) -> u32 {
        let non_empty_boxes = self.hash_map
                                .iter();

        non_empty_boxes.enumerate()
            .map(|(box_number, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(|(lens_number, lens)| {
                        let sum: u32 = (box_number as u32 + 1) * (lens_number as u32 + 1) * lens.focal_length as u32;
                        sum
                    })
                    .sum::<u32>()
            })
            .sum::<u32>()
            .try_into()
            .unwrap_or(0)
    }

    pub fn accumulate_hash(&self) -> i32 {
        return self.sequence
                .iter()
                .map(|step| Facility::hash(step))
                .sum()
    }
}

fn main() {
    let mut facility: Facility = Facility::load_from_file(FILE_NAME);
    // println!("Sum of hashes: {}", facility.accumulate_hash());

    facility.process_sequence();

    println!("{:#?}", facility.hash_map);
    println!("focusing power: {}", facility.get_focusing_power());
}

#[cfg(test)]
mod test {
    use crate::Facility;

    #[test]
    pub fn test_load_file() {
        let initialization_sequence = Facility::load_from_file("test_input.txt");
        assert_eq!(initialization_sequence.sequence, vec![
            "rn=1",
            "cm-",
            "qp=3",
            "cm=2",
            "qp-",
            "pc=4",
            "ot=9",
            "ab=5",
            "pc-",
            "pc=6",
            "ot=7"
        ])
    }

    #[test]
    pub fn test_hash() {
        let input_output: Vec<(String, i32)> = vec![      
            ("rn=1".to_string(), 30),
            ("cm-".to_string(), 253),
            ("qp=3".to_string(), 97),
            ("cm=2".to_string(), 47),
            ("qp-".to_string(), 14),
            ("pc=4".to_string(), 180),
            ("ot=9".to_string(), 9),
            ("ab=5".to_string(), 197),
            ("pc-".to_string(), 48),
            ("pc=6".to_string(), 214),
            ("ot=7".to_string(), 231),
        ];

        for (input, output) in input_output {
            assert_eq!(Facility::hash(&input), output);
        }
    }

    #[test]
    pub fn test_focusing_power() {
        let mut facility = Facility::load_from_file("test_input.txt");
        facility.process_sequence();
        assert_eq!(facility.get_focusing_power(), 145);
    }
}
