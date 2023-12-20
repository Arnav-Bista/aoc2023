pub struct AOC9 {

}


impl AOC9 {
        
    fn get_difference(numbers: &Vec<i32>) -> (Vec<i32>, bool) {
        let mut diff = Vec::new();
        let mut is_same = true;
        let prev = numbers[1] - numbers[0];
        for i in 1..numbers.len() {
            let new_diff = numbers[i] - numbers[i - 1];
            diff.push(new_diff);
            if new_diff != prev {
                is_same = false;
            }

        }

        (diff, is_same)
    }

    fn get_next(line: String) -> i32 {
        let mut levels: Vec<Vec<i32>> = Vec::new();
        let mut numbers: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let last = *numbers.last().unwrap();
        loop {
            let (diff, is_same) = AOC9::get_difference(&numbers);
            numbers = diff.clone();
            levels.push(diff);
            if is_same {
                break;
            }
        }
        let mut next_diff = 0;
        for level in levels {
            next_diff += level.last().unwrap();
        }
        last + next_diff
    }

    pub fn solve_p1(lines: Vec<String>) -> i32 {
        let mut total = 0;
        for line in lines {
            total += AOC9::get_next(line);
        }       
        total
    }

    fn get_prev(line: String) -> i32 {
        let mut levels: Vec<Vec<i32>> = Vec::new();
        let mut numbers: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let first = numbers[0];
        loop {
            let (diff, is_same) = AOC9::get_difference(&numbers);
            numbers = diff.clone();
            levels.push(diff);
            if is_same {
                break;
            }
        }
        let mut prev_diff = 0;
        for i in 0..levels.len() {
            prev_diff = levels[levels.len() - 1 - i][0] - prev_diff;
        } 

        first - prev_diff
    }

    pub fn solve_p2(lines: Vec<String>) -> i32 {
        let mut total = 0;
        for line in lines {
            total += AOC9::get_prev(line);
        }       
        total
    }
}
