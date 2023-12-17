use std::collections::HashMap;

use regex::Regex;


enum Direction {
    Left,
    Right
}


pub struct AOC8 {
    map: HashMap<String, (String, String)>,
    moves: Vec<Direction>,
}


impl AOC8 {



    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            moves: Vec::new(),
        }
    }

    fn parse_moves(&mut self, line: &str) {
        for char in line.chars() {
            match char {
                'L' => self.moves.push(Direction::Left),
                'R' => self.moves.push(Direction::Right),
                _ => ()
            }
        }
    }

    pub fn solve_p1(&mut self, lines: Vec<String>) -> u32 {
        let regex: Regex = Regex::new("(\\w+) = \\((\\w+), (\\w+)\\)").unwrap();
        self.parse_moves(&lines[0]);
        for line in &lines[2..] {
            let regex_match = regex.captures(&line).unwrap();
            // Construct an edge list kinda data structure
            // Kinda like a directed adjacency list
            self.map.insert(
                regex_match.get(1).unwrap().as_str().to_owned(),
                (
                    regex_match.get(2).unwrap().as_str().to_owned(),
                    regex_match.get(3).unwrap().as_str().to_owned(),
                ));
        }

        // Kinda a brute force solution?
        let mut count = 0;
        let mut current: &str = "AAA";
        loop {
            let edges = self.map.get(current).unwrap();
            let direction = &self.moves[count % self.moves.len()];
            match direction {
                Direction::Left => current = edges.0.as_str(),
                Direction::Right => current = edges.1.as_str(),
            }
            count += 1;
            if current == "ZZZ" {
                break;
            }
        }
        count as u32
    }
}
