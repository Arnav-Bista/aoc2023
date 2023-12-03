
pub struct AOC3 {
    non_symbol: u8,
    gear_symbol: u8
}


impl AOC3 {
    pub fn new() -> Self {
        Self {
            non_symbol: 46,
            gear_symbol: 42
        }   
    }

    fn is_number(&self, symbol: u8) -> bool {
        symbol <= 57 && 48 <= symbol
    }

    fn search(&self, lines: &mut Vec<Vec<u8>>, i: usize, j: usize) -> Vec<u32> {
        // At max, there could be 6 numbers attached to the symbol
        let mut numbers: Vec<u32> = Vec::with_capacity(6);

        for h in (i - 1)..(i + 1) + 1 {
            for k in (j - 1)..(j + 1) + 1 {
                if h == i && k == j || !self.is_number(lines[h][k]) {
                    continue;
                }
                numbers.push(self.get_number(lines, h, k));
            }
        }

        numbers
    }

    fn get_number(&self, lines: &mut Vec<Vec<u8>>, i: usize, j:usize) -> u32 {
        let mut number: u32 = 0;
        // 0 - 48
        // 9 - 57
        let mut h = j;
        let mut k = j;
        while h > 0 && self.is_number(lines[i][h - 1]) {
            h -= 1;
        }

        while k <= lines[i].len() && self.is_number(lines[i][k + 1]) {
            k += 1;
        } 

        for index in h..k + 1 {
            number += (lines[i][index] as u32 - 48) * 10_u32.pow((k - index) as u32);
            lines[i][index] = self.non_symbol;
        }
        // self.debug(lines);
        number
    }

    pub fn solve(&self, lines: &mut Vec<Vec<u8>>) -> u32 {
        let mut sum: u32 = 0;
        for i in 1..lines.len() - 1 {
            for j in 1..lines[i].len() - 1 {
                // 0 - 48
                // 9 - 57
                if lines[i][j] != self.non_symbol && !self.is_number(lines[i][j]) {
                    let nums = self.search(lines, i, j);
                    if lines[i][j] == self.gear_symbol && nums.len() == 2 {
                        sum += nums[0] * nums[1];
                    }
                    // else {
                    //     for num in nums {
                    //         sum += num;
                    //     }
                    // }
                }
            }
        }
        sum
    }
}
