use colored::Colorize;

struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        if self.x1 == self.x2 || self.y1 == self.y2 {
            return false;
        }
        return true;
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        print!("{},{} -> {},{}", self.x1, self.y1, self.x2, self.y2);
        Ok(())
    }
}

struct OceanMap {
    lines: Vec<Line>,
    map: Vec<usize>,
    width: usize,
    height: usize,
}

impl OceanMap {
    fn new(path: &str) -> Self {
        let mut width = 0;
        let mut height = 0;

        let lines: Vec<Line> = std::fs::read_to_string(path)
            .unwrap()
            .split("\n")
            .map(|l| {
                let line = l.split_once(" -> ").unwrap();
                let left = line.0.split_once(",").unwrap();
                let right = line.1.split_once(",").unwrap();

                let linedata = Line {
                    x1: left.0.parse().unwrap(),
                    y1: left.1.parse().unwrap(),
                    x2: right.0.parse().unwrap(),
                    y2: right.1.parse().unwrap(),
                };

                if linedata.x1 > width {
                    width = linedata.x1.to_owned()
                };
                if linedata.x2 > width {
                    width = linedata.x2.to_owned()
                };
                if linedata.y1 > height {
                    height = linedata.y1.to_owned()
                };
                if linedata.y2 > height {
                    height = linedata.y2.to_owned()
                };

                return linedata;
            })
            .collect();

        let map: Vec<usize> = vec![0; width * (height + 1)];

        Self {
            lines,
            width,
            height,
            map,
        }
    }

    fn clear(&mut self) {
        self.map = vec![0; self.width * (self.height + 1)];
    }

    fn generate_map(&mut self, allow_diagonal: bool) {
        for line in &self.lines {
            let mut skip = false;

            if line.is_diagonal() {
                if !allow_diagonal {
                    skip = true;
                }
            }

            if !skip {
                for (x, y) in line_drawing::Bresenham::new(
                    (line.x1 as i32, line.y1 as i32),
                    (line.x2 as i32, line.y2 as i32),
                ) {
                    let pixelnum = ((y as usize) * self.width) + x as usize;
                    self.map[pixelnum] = self.map[pixelnum] + 1;
                }
            }
        }
        println!("updating map");
    }

    fn plot(&self) {
        println!("plotting");

        for (i, val) in self.map.iter().enumerate() {
            if i % self.width == 0 {
                println!();
            }

            print!(
                "{}",
                val.to_string()
                    .truecolor((val * 100) as u8, (val * 50) as u8, (val * 200) as u8)
            );
        }
        println!();
    }

    fn count(&self) -> usize {
        self.map.iter().filter(|&v| *v > 1).count()
    }
}

fn process(filename: &str) -> (usize, usize) {
    let mut ocean = OceanMap::new(filename);
    ocean.generate_map(false);
    ocean.plot();

    let part1_answer = ocean.count();
    println!("part1: {}", part1_answer);

    ocean.clear();
    ocean.generate_map(true);
    ocean.plot();

    let part2_answer = ocean.count();
    println!("part2: {}", part2_answer);

    (part1_answer, part2_answer)
}

pub fn run() {
    process("src/advent2021/day05/sample.txt");
}

#[test]
fn test() {
    let sample = process("src/advent2021/day05/sample.txt");
    assert_eq!(sample, (5, 12));

    let input = process("src/advent2021/day05/input.txt");
    assert_eq!(input, (5147, 16925));
}
