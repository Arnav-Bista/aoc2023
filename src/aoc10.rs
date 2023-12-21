use std::collections::{HashMap, VecDeque};



enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {

    pub const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Self::Up
        }
    }

    pub fn get_coordinates(&self, original: &(usize, usize)) -> (usize, usize) {
        let mut new = original.clone();
        match self {
            Direction::Left => new.1 -= 1,
            Direction::Right => new.1 += 1,
            Direction::Up => new.0 -= 1,
            Direction::Down => new.0 += 1
        };
        new
    }

    pub fn is_valid(&self, tile: char) -> bool {
        match self {
            Direction::Left => tile == 'F' || tile == '-' || tile == 'L',
            Direction::Right => tile == '7' || tile == '-' || tile == 'J',
            Direction::Up => tile == '|' || tile == '7' || tile == 'F',
            Direction::Down => tile == '|' || tile == 'J' || tile == 'L'
        }
    }

    pub fn get_available_directions(tile: char) -> Vec<Direction> {
        let mut directions: Vec<Direction> = Vec::with_capacity(4);

        match tile {
            '-' => {
                directions.push(Self::Left);
                directions.push(Self::Right);
            }
            '|' => {
                directions.push(Self::Up);
                directions.push(Self::Down);
            }
            'J' => {
                directions.push(Self::Up);
                directions.push(Self::Left);
            }
            'F' => {
                directions.push(Self::Down);
                directions.push(Self::Right);
            }
            'L' => {
                directions.push(Self::Right);
                directions.push(Self::Up);
            }
            '7' => {
                directions.push(Self::Left);
                directions.push(Self::Down);
            }
            _ => ()

        }
        directions
    }
}

pub struct AOC10 {
    map: Vec<Vec<char>>,
    start: (usize, usize),
}

impl AOC10 {
    pub fn new() -> Self {
        Self {
            map: Vec::new(),
            start: (0,0)
        }
    }

    fn find_start(&mut self) {
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                if self.map[i][j] == 'S' {
                    // Start Found
                    self.start = (i, j);
                    return;
                }
            }
        }
    }

    // fn get_neighbours(&self, node: (usize, usize)) -> Vec<(usize, usize)> {
    //     let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(4);
    //     // Start Case
    //     if self.map[node.0][node.1] == 'S' {
    //         for direction in Direction::DIRECTIONS {
    //             let new_node = 
    //         }
    //     }
    //     neighbours
    // }
    //
    pub fn solve_p1(&mut self, lines: Vec<String>) -> u32 {
        // INPUTS ARE PRE PADDED
        // Construct graph
        self.map = lines.iter().map(|line| line.chars().collect()).collect();
        // Find starting
        self.find_start();
        let mut visited: HashMap<(usize, usize), u32> = HashMap::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut level_queue: VecDeque<u32> = VecDeque::new();
        // BFS
        queue.push_back(self.start);
        level_queue.push_back(0);
        visited.insert(self.start, 0);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let level = level_queue.pop_front().unwrap();
            // Start Edge Case
            if self.map[node.0][node.1] == 'S' {
                for direction in Direction::DIRECTIONS {
                    let new = direction.get_coordinates(&node);
                    if direction.is_valid(self.map[new.0][new.1]) && visited.get(&new).unwrap_or(&u32::MAX) > &level {
                        visited.insert(new, level + 1);
                        level_queue.push_back(level + 1);
                        queue.push_back(new);
                    }
                }
            }
            // Any other Node
            else {
                let directions = Direction::get_available_directions(self.map[node.0][node.1]);
                for direction in directions {
                    let new = direction.get_coordinates(&node);
                    if direction.is_valid(self.map[new.0][new.1]) && visited.get(&new).unwrap_or(&u32::MAX) > &level {
                        visited.insert(new, level + 1);
                        level_queue.push_back(level + 1);
                        queue.push_back(new);
                    }
                }
            }
        }
        // println!("{:?}", visited);
        // println!("{}",self.map[0].len());
        // // DEBUG
        // let mut v: Vec<Vec<char>> = vec![vec!['.'; self.map[0].len()]; self.map.len()];
        // for key in visited.keys() {
        //     println!("{:?}", key);
        //     v[key.0][key.1] = self.map[key.0][key.1];
        // }
        // for vecs in v {
        //     for c in vecs {
        //         print!("{}", c);
        //     }
        //     println!();
        // }
        *visited.values().max().unwrap()
    }

    pub fn solve_p2(&mut self, lines: Vec<String>) -> u32 {
        // INPUTS ARE PRE PADDED
        // Construct graph
        self.map = lines.iter().map(|line| line.chars().collect()).collect();
        // Find starting
        self.find_start();
        let mut visited: HashMap<(usize, usize), u32> = HashMap::new();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut level_queue: VecDeque<u32> = VecDeque::new();
        // BFS
        queue.push_back(self.start);
        level_queue.push_back(0);
        visited.insert(self.start, 0);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let level = level_queue.pop_front().unwrap();
            // Start Edge Case
            if self.map[node.0][node.1] == 'S' {
                for direction in Direction::DIRECTIONS {
                    let new = direction.get_coordinates(&node);
                    if direction.is_valid(self.map[new.0][new.1]) && visited.get(&new).unwrap_or(&u32::MAX) > &level {
                        visited.insert(new, level + 1);
                        level_queue.push_back(level + 1);
                        queue.push_back(new);
                    }
                }
            }
            // Any other Node
            else {
                let directions = Direction::get_available_directions(self.map[node.0][node.1]);
                for direction in directions {
                    let new = direction.get_coordinates(&node);
                    if direction.is_valid(self.map[new.0][new.1]) && visited.get(&new).unwrap_or(&u32::MAX) > &level {
                        visited.insert(new, level + 1);
                        level_queue.push_back(level + 1);
                        queue.push_back(new);
                    }
                }
            }
        }


        // PART 2
        // Visited has the path
        // Just avoid them
        // Also avoid paddings
        0
    }
}
