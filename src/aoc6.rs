pub struct AOC6 {
    time: Vec<f32>,
    distance: Vec<f32>
}


impl AOC6 {
    
    pub fn new() -> Self {
        Self {
            time: Vec::new(),
            distance: Vec::new()
        }
    }
        
    
    fn setup(&mut self, lines: &Vec<String>) {
        let times = lines[0].split(':').next_back().unwrap().split_whitespace();
        let distances = lines[1].split(':').next_back().unwrap().split_whitespace();

        for time in times {
            self.time.push(time.parse().unwrap());
        }

        for distnace in distances {
            self.distance.push(distnace.parse().unwrap());
        }

    }

    pub fn solve_p1(&mut self, lines: Vec<String>) -> u32 {
        self.setup(&lines);
        /*
        * S = D / T
        * x = D / (T - x)
        * xT - x^2 = D
        * x^2 - xT + D = 0 QUADRATIC
        * Possible answers is basically the integer range between the two roots.
        * (T +- sqrt(D)) / 2 -> two Roots
        * Difference of roots: Sqrt(D) / A
        * D = T^2 - 4 * distance
        */
        let mut product = 1;
        let iterator = self.time.iter().zip(self.distance.iter());
        for (time, distance) in iterator {
            let first: f32 = (time - (time * time - 4.0 * distance).powf(0.5)) / 2.0;
            let second: f32 = (time + (time * time - 4.0 * (distance + 0.1)).powf(0.5)) / 2.0;
            // println!("{} {} {}", first, second, second.floor() - first.floor());
            product *= (second.floor() - first.floor()) as u32;
        }
        product
    }
    
    fn setup_p2(&self, lines: Vec<String>) -> (f64, f64) {
        let times: Vec<&str> = lines[0].split(':').next_back().unwrap().split_whitespace().collect();
        let distances: Vec<&str> = lines[1].split(':').next_back().unwrap().split_whitespace().collect();
        (times.join("").parse().unwrap(), distances.join("").parse().unwrap())
    }

    pub fn solve_p2(&mut self, lines: Vec<String>) -> u32 {
        let (time, distance) = self.setup_p2(lines);
        println!("{:?}", (time, distance));
        let first = (time - (time * time - 4.0 * distance).powf(0.5)) / 2.0;
        let second = (time + (time * time - 4.0 * (distance + 0.1)).powf(0.5)) / 2.0;
        // println!("{} {} {}", first, second, second.floor() - first.floor());
        (second.floor() - first.floor()) as u32
    }
}
