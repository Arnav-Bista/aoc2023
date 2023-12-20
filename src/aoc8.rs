use std::{collections::{HashMap, HashSet}, fmt::Display, mem::swap, arch::x86_64::_MM_FROUND_CUR_DIRECTION};

use regex::Regex;


enum Direction {
    Left,
    Right
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Direction::Left => "L",
            Direction::Right => "R"
        };
        write!(f, "{}", res)
    }
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



    fn gcd(mut a: u32, mut b:u32) -> u32 {
        if a < b {
            swap(&mut a,&mut b);
        }
        if a == b || a % b == 0 {
            return b
        }
        AOC8::gcd(b, a % b)
    }

    fn lcm(a: u32, b: u32) -> u32 {
        a * b / AOC8::gcd(a, b)
    }

    fn get_hash(node: String, edges: (String, String), direction: &Direction) -> String {
        if edges.0 == edges.1 {
            return node + "B";
        }
        match direction {
            Direction::Left => node + "L",
            Direction::Right => node + "R",
        }
    }


    pub fn solve_p2(&mut self, lines: Vec<String>) -> u32 {
        let regex: Regex = Regex::new("(\\w+) = \\((\\w+), (\\w+)\\)").unwrap();
        self.parse_moves(&lines[0]);
        let mut nodes: Vec<String> = Vec::new();
        for line in &lines[2..] {
            let regex_match = regex.captures(&line).unwrap();
            let node = regex_match.get(1).unwrap().as_str().to_owned();
            if node.chars().last().unwrap() == 'A' {
                nodes.push(node.to_owned());
            }
            self.map.insert(
                node,
                (
                    regex_match.get(2).unwrap().as_str().to_owned(),
                    regex_match.get(3).unwrap().as_str().to_owned(),
                ));
        }
        
        // Each Start will have constant + loop steps to get to the dest
        // We need to get the LCM of the loop step + constant
        let mut distance_to_loop = vec![0; nodes.len()];
        let mut loop_length = vec![0; nodes.len()];
        let mut distance_to_z = vec![0; nodes.len()];

        for i in 0..nodes.len() {

            let mut visited: HashMap<String, u32> = HashMap::new();
            let mut current_node = nodes[i].to_string();

            let mut node_hash;
            
            loop {
                let edges = self.map.get(&current_node).unwrap();
                let direction = &self.moves[distance_to_loop[i] as usize % self.moves.len()];
                node_hash = current_node + &(distance_to_loop[i] as usize % self.moves.len()).to_string();
                // node_hash = AOC8::get_hash(current_node.clone(), edges.clone(), direction);

                current_node = match direction {
                    Direction::Left => edges.0.to_owned(),
                    Direction::Right => edges.1.to_owned()
                };

                //
                // if visited.contains_key(&node_hash) {
                //     break
                // }
                visited.insert(node_hash, distance_to_loop[i]);
                distance_to_loop[i] += 1;

                if current_node.ends_with('Z') {
                    println!("FOUND");
                    distance_to_z[i] = distance_to_loop[i];
                    break;
                }


            }
            // let length = visited.get(&node_hash).unwrap();
            // loop_length[i] = distance_to_loop[i] - length;
            // distance_to_loop[i] = *length;
            // println!("{:?}", visited);
        }
        println!("{:?}", distance_to_z);
        println!("{:?}", distance_to_loop);
        println!("{:?}", loop_length);
        

        // We SHOULD have the length of the loop as well as the length till entering the loop
        // let lcm = loop_length.iter().copied().reduce(|a, b| AOC8::lcm(a, b)).unwrap();
        // let max_constant = distance_to_loop.iter().max().unwrap();
        let huh_lcm = distance_to_z.iter().copied().reduce(|a, b| AOC8::lcm(a, b)).unwrap();
        huh_lcm 
    }
}
