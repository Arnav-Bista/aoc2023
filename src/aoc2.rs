
use regex::Regex;

pub struct AOC2 {
    red: u32,
    blue: u32,
    green: u32,
    regex: Regex,
    separator: Regex
}


impl AOC2 {

    pub fn new() -> Self {
        Self {
            red: 12,
            green: 13,
            blue: 14,
            regex: Regex::new("Game (\\d+):(.*)").unwrap(),
            separator: Regex::new("\\s*(\\d+)\\s*(\\w+)\\s*").unwrap()
        }
    }

    pub fn determine_valid(&mut self, line: &str) -> Option<u32>{
        let sets = line.split(';');
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in sets {
            let games = set.split(',');
            for game in games {
                let res = self.separator.captures(game).unwrap();
                let num: u32 = res.get(1).unwrap().as_str().parse().unwrap();
                let color = res.get(2).unwrap().as_str();
                match color {
                    "red" => {
                        min_red = u32::max(min_red, num);
                        // if num > self.red {
                        //     return None
                        // }
                    }
                    "blue" => {
                        min_blue = u32::max(min_blue, num);
                        // if num > self.blue {
                        //     return None;
                        // }
                    }
                    "green" => {
                        min_green = u32::max(min_green, num);
                        // if num > self.green {
                        //     return None;
                        // }
                    }
                    _ => ()

                }
            }
        }
        Some(min_red * min_green * min_blue) 
    }   

    pub fn solve(&mut self, lines: Vec<String>) -> u32 {
        let mut sum: u32 = 0;
        // let mut game_id: u32;
        for line in lines {
            let res = self.regex.captures(&line).unwrap();
            // game_id = res.get(1).unwrap().as_str().parse().unwrap();
            let game_value = res.get(2).unwrap().as_str();
            match self.determine_valid(game_value) {
                Some(val) => {
                    sum += val
                }
                None => ()
            }
        }

        sum
    }
}
