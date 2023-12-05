pub struct AOC5 {
    values: Vec<u64>,
    values_p2: Vec<(u64, u64)>,
    level: Vec<usize>,
}


impl AOC5 {

    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            values_p2: Vec::new(),
            level: Vec::new()
        }
    }

    fn process_seeds(&mut self, line: &str) {
        // Initially, our seeds are our original values.
        let mut iterator = line.split(':');
        iterator.next();
        let numbers = iterator.next().unwrap().split_whitespace();
        for number in numbers {
            self.values.push(number.parse().unwrap());
        }
        self.level = vec![1; self.values.len()];
    }

    fn get_numbers(&self, line: &str) -> (u64, u64, u64) {
        let mut numbers = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());
        (numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap())
    }

    fn process_map(&mut self, line: &str, level: usize) {
        let numbers: (u64, u64, u64) = self.get_numbers(line);
        for i in 0..self.values.len() {
            if self.level[i] <= level && numbers.1 <= self.values[i] && self.values[i] < numbers.1 + numbers.2 {
                let diff = self.values[i] - numbers.1;
                self.values[i] = numbers.0 + diff;
                self.level[i] = level + 1;
            }
        }
    }

    pub fn solve(&mut self, lines: Vec<String>) -> u64 {
        // Line 1 is aways the seeds
        self.process_seeds(&lines[0]);
        let mut level = 0;
        for i in 1..lines.len() {
            if lines[i] == "" {
                // Skip empty lines and header lines
                continue;
            }
            if lines[i].contains(':') {
                level += 1;
                continue;
            }
            // Now each input will be a MAP 
            // DEST SOURCE LENGTH
            self.process_map(&lines[i], level);
        }
        *self.values.iter().min().unwrap()
    }




    fn process_seeds_p2(&mut self, line: &str) {
        // Initially, our seeds are our original values.
        let mut iterator = line.split(':');
        iterator.next();
        let mut numbers = iterator.next().unwrap().split_whitespace();
        while let (Some(v1), Some(v2)) = (numbers.next(), numbers.next()) {
            self.values_p2.push((v1.parse().unwrap(),v2.parse().unwrap()));
        }
        self.level = vec![1; self.values_p2.len()];
    }

    fn process_map_p2(&mut self, line: &str, level: usize) {
        let numbers: (u64, u64, u64) = self.get_numbers(line);
        for i in 0..self.values_p2.len() {
            let val: (u64, u64) = self.values_p2[i];
            if self.level[i] <= level {
                // Check if the ranges overlap
                if val.0 <= numbers.1 + numbers.2 && numbers.1 <= val.0 + val.1 {
                    self.level[i] = level;
                    // Case 1 - Map Range consumes Seed Range or equal
                    if val.0 >= numbers.1 && val.1 <= numbers.2 {
                        let start_diff = val.0 - numbers.1;
                        let range_diff = val.1.min(numbers.2);
                        let new_range = (numbers.0 + start_diff, range_diff);
                        self.values_p2[i] = new_range;
                    }
                    // Case 2 - Seed Range starts below Map range
                    else if val.0 < numbers.1 {
                        let start_diff = numbers.1 - val.0;
                        let seed_range = (val.0 + val.1) - numbers.2;
                        let new_range = (numbers.0, seed_range);
                        let leftover_range = (val.0, start_diff);
                        self.values_p2[i] = new_range;
                        self.values_p2.push(leftover_range);
                        if seed_range != 0 {
                            self.level.push(level);
                        }
                    }
                    // Case 3 - Seed Range exceeds Map Range
                    else if val.0 > numbers.1 {
                        let start_diff = val.0 - numbers.1;
                        let map_range_end = numbers.1 + numbers.2;
                        let end_diff = val.0 + val.1 - map_range_end;
                        let new_range = (numbers.0 + start_diff, end_diff);
                        let leftover_range = (map_range_end, end_diff);
                        self.values_p2[i] = new_range;
                        self.values_p2.push(leftover_range);
                        if end_diff != 0 {
                            self.level.push(level);
                        }
                    }
                }
            }
        }
    }

    pub fn solve_p2(&mut self, lines:Vec<String>) -> u64 {
        // Line 1 is aways the seeds
        self.process_seeds_p2(&lines[0]);
        let mut level = 0;
        for i in 1..lines.len() {
            if lines[i] == "" {
                // Skip empty lines and header lines
                continue;
            }
            if lines[i].contains(':') {
                level += 1;
                continue;
            }
            // Now each input will be a MAP 
            // DEST SOURCE LENGTH
            self.process_map_p2(&lines[i], level);
            println!("Map Processed {}/{} Length: {}", i, lines.len(), self.values_p2.len());
        }
        let mut min: u64 = u64::MAX;
        for (val,_) in &self.values_p2 {
            min = min.min(*val);
        }

        min
    }
}
