use std::u32;

pub struct AOC1 {
    pointers: [usize; 9],
    numbers: Vec<Vec<u32>>,
}

impl AOC1 {
    pub fn new() -> Self {
        let values = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let mut numbers: Vec<Vec<u32>> = Vec::with_capacity(9);
        for i in 0..9 {
            numbers.push(values[i].to_owned().chars().map(|c| c as u32).collect());
        }
        Self {
            pointers: [0; 9],
            numbers,
        }
    }

    pub fn solve(&mut self, input: Vec<String>) -> u32 {
        let mut sum: u32 = 0;
        for line in input {
            let mut found: bool = false;
            let mut number: u32 = 0;
            for char in line.chars() {
                let ascii_value = char as u32;
                match self.check_char(ascii_value) {
                    Some(num) => {
                        number = num;
                        if !found {
                            found = true;
                            sum += number * 10;
                        }
                    }
                    None => {
                        if 47 < ascii_value && ascii_value < 58 {
                            number = char as u32 - 48;
                            if !found {
                                found = true;
                                sum += number * 10;
                            }
                        }
                    }
                }
            }
            sum += number;
        }
        sum
    }

    fn check_char(&mut self, char: u32) -> Option<u32> {
        let mut result: Option<u32> = None;
        for i in 0..9 {
            if char == self.numbers[i][self.pointers[i]] {
                self.pointers[i] += 1;
            } else if char == self.numbers[i][0] {
                self.pointers[i] = 1;
            } else {
                self.pointers[i] = 0;
            }
            if self.pointers[i] == self.numbers[i].len() {
                self.pointers[i] = 0;
                // self.pointers = [0;9];
                result = Some((i + 1) as u32);
            }
        }
        result
    }
}
