pub struct AOC4 {
    makeshift_map: Vec<usize>,
    makeshift_map_p2: Vec<usize>
}


impl AOC4 {

    pub fn new() -> Self {
        Self {
            makeshift_map: vec![0; 101],
            makeshift_map_p2: Vec::new()
        }
    }

    fn get_numbers(&self, numbers: &str) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        let numbers = numbers.split_whitespace();
        for string_number in numbers {
            let mut formed_number: usize = 0;
            let length = string_number.len();
            for (i, char) in string_number.chars().enumerate() {
                formed_number += (char as usize - '0' as usize) * 10_usize.pow((length - i - 1) as u32);
            }
            res.push(formed_number);
        }
        res
    }

    fn process(&mut self, winning_numbers: &str, level: usize) {
        for numbers in self.get_numbers(winning_numbers) {
            self.makeshift_map[numbers] = level;
        }
    }

    fn get_winnings(&mut self, current_numers: &str, level: usize)  -> u32 {
        let mut total: u32 = 0;
        for number in self.get_numbers(current_numers) {
            if self.makeshift_map[number] == level {
                total += 1;
            }
        }
        total
    }

    pub fn solve(&mut self, lines: Vec<String>) -> u32 {
        let mut total: u32 = 0;
        let length = lines.len();
        self.makeshift_map_p2 = vec![0; (length + 1).max(100)];
        for (i, line) in lines.into_iter().enumerate() {
            let i = i + 1;
            // The inputs are always in this specific structure
            let mut iter = line.split(':');
            iter.next();
            let mut cards = iter.next().unwrap().split('|');
            let winning_numbers = cards.next().unwrap();
            let current_numbers = cards.next().unwrap();

            self.process(winning_numbers, i);
            let num_matching = self.get_winnings(current_numbers, i); 
            if num_matching != 0 {
                // Part 1
                // total += 2_u32.pow(num_matching - 1);
            }

            // Part 2
            self.makeshift_map_p2[i] += 1;
            for j in (i + 1)..(i + 1 + num_matching as usize) {
                self.makeshift_map_p2[j] += self.makeshift_map_p2[i];
            }
        }
        for num in &self.makeshift_map_p2 {
            total += *num as u32;
        }
        total
    }
}


