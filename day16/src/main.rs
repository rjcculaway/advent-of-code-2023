//--------------------------------------------------------------------------------
// Day 16: The Floor Will Be Lava
// Rene Jotham Culaway
//--------------------------------------------------------------------------------

use std::collections::HashSet;

const FILE_NAME: &'static str = "input.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    position: (i32, i32),
    direction: char
}

impl Beam {
    pub fn traverse(&mut self) {
        match self.direction {
            '>' => {
                self.position.0 += 1;
            },
            '<' => {
                self.position.0 -= 1;
            },
            '^' => {
                self.position.1 -= 1;
            },
            'v' => {
                self.position.1 += 1;
            },
            _ => unreachable!()

        }
    }
}

#[derive(Debug, Clone)]
enum MapElement {
    Passthrough,
    Mirror {
        symbol: char
    }
}

struct Cave {
    grid: Vec<Vec<MapElement>>,
    width: usize,
    height: usize,
}

impl Cave {
    pub fn load_from_file(file_name: &str) -> Self {
        let mut grid: Vec<Vec<MapElement>> = Vec::new();
        

        if let Ok(file_contents) = std::fs::read_to_string(file_name) {
            for line in file_contents.lines() {
                grid.push(
                    line
                    .as_bytes()
                    .iter()
                    .map(|byte| {
                        match *byte as char {
                            '.' => {
                                MapElement::Passthrough {}
                            },
                            c @ '/' | c @ '|' | c @  '-' | c @ '\\' => {
                                MapElement::Mirror { symbol: c }
                            },
                            _ => unreachable!(),
                        }
                    })
                    .collect()
                );
            }
        }

        let width = grid[0].len();
        let height = grid.len();

        Cave { grid, width, height }
    }

    fn beam_incident(beam: Beam, tile: &MapElement, directions_reflected: &mut HashSet<char>) -> Vec<Beam> {
        let mut resulting_beams: Vec<Beam> = vec![];
        match tile {
            MapElement::Passthrough {} => {
                resulting_beams.push(beam);
            }
            MapElement::Mirror { symbol } => {
                let beam_direction = beam.direction;
                if directions_reflected.contains(&beam_direction) {
                    match (beam_direction, symbol) {
                        ('>', '/') | ('<', '\\') => {
                            resulting_beams.push(Beam { direction: '^', position: beam.position });
                        },
                        ('>', '\\') | ('<', '/') => {
                            resulting_beams.push(Beam { direction: 'v', position: beam.position });
                        },
                        ('v', '/') | ('^', '\\') => {
                            resulting_beams.push(Beam { direction: '<', position: beam.position });
                        },
                        ('^', '/') | ('v', '\\') => {
                            resulting_beams.push(Beam { direction: '>', position: beam.position });
                        },
                        ('>', '|') | ('<', '|') => {
                            Vec::extend_from_slice(&mut resulting_beams, &[Beam { direction: '^', position: beam.position }, Beam { direction: 'v', position: beam.position }]);
                        },
                        ('^', '-') | ('v', '-') => {
                            Vec::extend_from_slice(&mut resulting_beams, &[Beam { direction: '<', position: beam.position }, Beam { direction: '>', position: beam.position }]);
                        },
                        ('^', '|') | ('v', '|') | ('<', '-') | ('>', '-') => {
                            resulting_beams.push(beam);
                        },
                        _ => unreachable!(),
                    }
                    directions_reflected.remove(&beam_direction);
                }
            }
        }

        return resulting_beams;
    }

    pub fn trace_beams(&self, starting_beam: Option<Beam>) -> HashSet<(u32, u32)> {

        let starting_beam: Beam = starting_beam.unwrap_or(Beam { position: (0, 0), direction: '>' });

        let mut energized: HashSet<(u32, u32)> = HashSet::new();
        let mut beams: Vec<Beam> = Vec::from( [ starting_beam ] );
        let mut reflection_status: Vec<Vec<HashSet<char>>> = Vec::new();

        for _ in 0..self.height {
            let mut row = Vec::new();
            for _ in 0..self.width {
                row.push(HashSet::from(['^', 'v', '>', '<']));
            }
            reflection_status.push(row)
        }

        loop {
            let mut next_beams: Vec<Beam> = vec![];
            while !beams.is_empty() {
                let beam = beams.pop().unwrap();
                // println!("{:?}", beam);
                let (x, y) = beam.position;
                energized.insert((x as u32, y as u32));

                let mut resulting_beams = Cave::beam_incident(beam, &self.grid[y as usize][x as usize], &mut reflection_status[y as usize][x as usize]);

                for resulting_beam in resulting_beams.iter_mut() {
                    resulting_beam.traverse();
                }
                next_beams.extend(resulting_beams
                    .iter()
                    .filter(|beam| { 
                        let (x, y) = beam.position;
                        if x < 0 || x >= self.width.try_into().unwrap_or(0) {
                            return false;
                        } 

                        if y < 0 || y >= self.height.try_into().unwrap_or(0) {
                            return false;
                        }

                        return true;
                     })
                     .cloned()
                );
            }
            // self.print_energized(&energized);
            // println!();

            beams.extend(next_beams.drain(..));
            if beams.is_empty() {
                break;
            }
        }
        energized
    }

    pub fn get_max_energized(&self) -> u32 {
        let max_top_x: usize = (0..self.width)
                                    .map(|starting_x|
                                        self.trace_beams(Some(Beam { position: (starting_x.try_into().unwrap_or(0), 0), direction: 'v' }))
                                        .len())
                                    .max()
                                    .unwrap_or(0);
        let max_bottom_x: usize = (0..self.width)
                                    .map(|starting_x|
                                        self.trace_beams(Some(Beam { position: (starting_x.try_into().unwrap_or(0), self.height.try_into().unwrap_or(0) - 1), direction: '^' }))
                                        .len())
                                    .max()
                                    .unwrap_or(0);
        let max_left_y: usize = (0..self.height)
                                    .map(|starting_y|
                                        self.trace_beams(Some(Beam { position: (0, starting_y.try_into().unwrap_or(0)), direction: '>' }))
                                        .len())
                                    .max()
                                    .unwrap_or(0);
        let max_right_y: usize = (0..self.height)
                                    .map(|starting_y|
                                        self.trace_beams(Some(Beam { position: (self.width.try_into().unwrap_or(0) - 1, starting_y.try_into().unwrap_or(0)), direction: '<' }))
                                        .len())
                                    .max()
                                    .unwrap_or(0);

        max_top_x.max(max_bottom_x).max(max_left_y).max(max_right_y).try_into().unwrap_or(0)
        
    }

    pub fn print_energized(&self, energized: &HashSet<(u32, u32)>) {
        for y in 0..self.height {
            for x in 0..self.width {
                if energized.contains(&(x.try_into().unwrap_or(0), y.try_into().unwrap_or(0))) {
                    print!("{}", '#');
                } else {
                    print!("{}", '.');
                }
            }
            println!();
        }
    }

}

fn main() {
    let cave: Cave = Cave::load_from_file(FILE_NAME);
    let energized = cave.trace_beams(None);
    cave.print_energized(&energized);
    println!("Number of energized locations: {}", energized.len());

    let max_energized = cave.get_max_energized();
    println!("Max number of energized locations: {}", max_energized);
}

#[cfg(test)]
mod test {
    use crate::Cave;

    #[test]
    fn test_trace_beams() {
        let cave: Cave = Cave::load_from_file("test_input.txt");
        let energized = cave.trace_beams(None);

        assert_eq!(energized.len(), 46);
    }

    #[test]
    fn test_max_energized() {
        let cave: Cave = Cave::load_from_file("test_input.txt");

        assert_eq!(cave.get_max_energized(), 51);
    }
}