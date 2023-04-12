use colored::Colorize;

// https://docs.rs/pathfinding/latest/pathfinding/
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, hill: &Hill) -> Vec<Pos> {
        let &Pos(x, y) = self;

        // println!("\nsuccessors {:?}", (x, y));

        if x < 0 {
            return Vec::new();
        };
        if y < 0 {
            return Vec::new();
        };

        let coord = Coord {
            x: self.0.clone() as usize,
            y: self.1.clone() as usize,
        };

        // println!("coord {:?}", coord);

        let possible = hill.get_possible_steps(&coord);

        let options = possible
            .iter()
            .map(|p| Pos(p.x as i32, p.y as i32))
            .collect();

        // println!("{:?}", options);

        return options;

        // vec![
        //     Pos(x + 1, y + 2),
        //     Pos(x + 1, y - 2),
        //     Pos(x - 1, y + 2),
        //     Pos(x - 1, y - 2),
        //     Pos(x + 2, y + 1),
        //     Pos(x + 2, y - 1),
        //     Pos(x - 2, y + 1),
        //     Pos(x - 2, y - 1),
        // ]
    }
}

#[derive(Clone, Debug)]
struct Hill {
    map: Vec<MapPoint>,
    /** east-west size */
    width: usize,
    /** north-south size */
    height: usize,

    route: Vec<Coord>,
}

#[derive(Clone, Copy, Debug)]
struct MapPoint {
    symbol: char,
    elevation: u8,
    is_start: bool,
    is_end: bool,
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn char_to_elevation(c: char) -> u8 {
    let mut val = c as u8;

    if c == 'S' {
        val = 'a' as u8;
    }

    if c == 'E' {
        val = 'z' as u8;
    }

    return (val - ('a' as u8));
}

impl Hill {
    fn load(path: &str) -> Hill {
        let data = std::fs::read_to_string(path).unwrap();

        let width = data.lines().next().unwrap().len();
        let height: usize = data.lines().count();

        let wi32 = width as i32;
        let hi32 = height as i32;

        let mut map: Vec<MapPoint> = data
            .lines()
            .flat_map(|l| {
                l.chars().map(move |c| {
                    return MapPoint {
                        symbol: c,
                        elevation: char_to_elevation(c),
                        is_start: c == 'S',
                        is_end: c == 'E',
                        x: 0,
                        y: 0,
                    };
                })
            })
            .collect();

        map = map
            .iter_mut()
            .enumerate()
            .map(|(i, m)| {
                m.x = (i as i32) % wi32;
                m.y = (i as i32) / wi32;

                return m.clone();
            })
            .collect();

        Hill {
            map,
            width,
            height,
            route: Vec::new(),
        }
    }

    fn is_new_step(&self, coord: &Coord) -> bool {
        let routeCoord: Vec<Coord> = self
            .route
            .iter()
            .filter(|r| r.x == coord.x && r.y == coord.y)
            .map(|r| r.clone())
            .collect();

        routeCoord.len() == 0
    }

    fn print(&self) {
        let mut output: String = "".to_string();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;

                let loc = &self.map[index];

                let elevation = loc.elevation;

                if loc.is_start {
                    // print!("{}", loc.symbol.to_string().bright_yellow());

                    output = format!("{output}{}", loc.symbol.to_string().bright_yellow());
                }

                if loc.is_end {
                    // print!("{}", loc.symbol.to_string().bright_green());
                    output = format!("{output}{}", loc.symbol.to_string().bright_green());
                }

                let routeCoord: Vec<Coord> = self
                    .route
                    .iter()
                    .filter(|r| r.x == x && r.y == y)
                    .map(|r| r.clone())
                    .collect();

                if !loc.is_start && !loc.is_end {
                    if routeCoord.len() > 0 {
                        // print!("{}", loc.symbol.to_string().truecolor(100, 100, 100));
                        output = format!(
                            "{output}{}",
                            loc.symbol.to_string().truecolor(200, 200, 200)
                        );
                    } else {
                        // print!("{}", loc.symbol.to_string().truecolor(100, 100, 100));
                        output = format!(
                            "{output}{}",
                            loc.symbol.to_string().truecolor(100, 100, 100)
                        );
                    }
                }

                // print!("{}", elevation)
            }
            output = format!("{output}\n");
        }

        println!("{}", output);
    }

    fn get_map_at_coord(&self, coord: &Coord) -> Option<MapPoint> {
        let index: usize = coord.y * self.width + coord.x;

        if index < 0 {
            return None;
        }
        if index >= self.map.len() - 1 {
            return None;
        }

        return Some(self.map[index]);
    }

    fn coord_to_index(&self, coord: Coord) -> usize {
        return coord.y * self.width + coord.x;
    }

    fn index_to_xy(&self, index: usize) -> Coord {
        Coord {
            x: index % self.width,
            y: index / self.width,
        }
    }

    fn check_add_possible(
        &self,
        current_map: MapPoint,
        possible: &mut Vec<Coord>,
        x: isize,
        y: isize,
    ) {
        if x < 0 {
            return;
        }
        if y < 0 {
            return;
        }
        if x > self.width as isize - 1 {
            return;
        }
        if y > self.height as isize - 1 {
            return;
        }

        let c = Coord {
            x: x as usize,
            y: y as usize,
        };

        if let Some(f) = self.get_map_at_coord(&c) {
            if f.elevation == current_map.elevation
                || f.elevation == current_map.elevation + 1
                || f.elevation < current_map.elevation
            {
                possible.push(c);
            }
        }
    }

    /** get possible lower elevation steps */
    fn get_possible_steps(&self, coord: &Coord) -> Vec<Coord> {
        // println!("coord: {:?}", coord);

        let current_map = match self.get_map_at_coord(coord) {
            Some(x) => x,
            None => return Vec::new(),
        };

        // println!("current_map: {:?}", current_map);

        let mut possible: Vec<Coord> = Vec::new();

        // north
        self.check_add_possible(
            current_map,
            &mut possible,
            coord.x as isize,
            coord.y as isize - 1,
        );
        // south
        self.check_add_possible(
            current_map,
            &mut possible,
            coord.x as isize,
            coord.y as isize + 1,
        );
        // east
        self.check_add_possible(
            current_map,
            &mut possible,
            coord.x as isize + 1,
            coord.y as isize,
        );
        // west
        self.check_add_possible(
            current_map,
            &mut possible,
            coord.x as isize - 1,
            coord.y as isize,
        );

        let possible_steps: Vec<Coord> = possible
            .iter()
            .filter(|f| self.is_new_step(f))
            .map(|p| p.clone())
            .collect();

        // println!("possible_steps: {:?}", possible_steps);

        possible_steps
    }

    fn get_start(&self) -> Coord {
        self.index_to_xy(self.map.iter().position(|i| i.symbol == 'S').unwrap())
    }

    fn get_end(&self) -> Coord {
        self.index_to_xy(self.map.iter().position(|i| i.symbol == 'E').unwrap())
    }

    fn part1(&self) -> usize {
        let mut goal = Pos(self.get_end().x as i32, self.get_end().y as i32);
        let mut start = Pos(self.get_start().x as i32, self.get_start().y as i32);

        println!("start: {:?}", start);
        println!("goal: {:?}", goal);
        let result = bfs(&start, |p| p.successors(self), |p| *p == goal);

        println!("start: {:?}", start);
        println!("goal: {:?}", goal);

        println!("result: {:?}", result);

        let steps = result.unwrap().iter().count();
        println!("steps: {}", steps);
        return steps - 1;
    }

    fn part2(&self) -> usize {
        // loop over every location with "a"; Find shortest route.

        let mut start_locations: Vec<MapPoint> = Vec::new();

        self.map.iter().for_each(|i| {
            if i.symbol == 'a' {
                start_locations.push(i.clone());
            }
        });

        let mut lowest = 99999999;

        let shortest: Vec<usize> = start_locations
            .iter()
            .map(|s| {
                let mut goal = Pos(self.get_end().x as i32, self.get_end().y as i32);
                let mut start = Pos(s.x as i32, s.y as i32);

                let result = bfs(&start, |p| p.successors(self), |p| *p == goal);

                let steps = match result {
                    Some(x) => x.iter().count(),
                    None => 9999999,
                } - 1;

                if steps < lowest {
                    println!("{} {:?}", steps, s);
                    lowest = steps;
                }

                return steps;
            })
            .collect();

        let answer = shortest.iter().min().unwrap();

        println!("answer: {:?}", answer);

        return answer.clone();
    }
}

#[allow(dead_code)]
pub fn run() {
    let mut hill = Hill::load("src/advent2022/day12/input.txt");

    hill.print();
    hill.part2();

    // hill.find_shortest_path();
}

#[test]
fn sample_part1() {
    let mut hill = Hill::load("src/advent2022/day12/sample.txt");
    assert_eq!(hill.part1(), 31);
}

#[test]
fn sample_part2() {
    let mut hill = Hill::load("src/advent2022/day12/sample.txt");

    assert_eq!(hill.part2(), 29);
}

#[test]
fn input() {
    let mut hill = Hill::load("src/advent2022/day12/input.txt");
    assert_eq!(hill.part1(), 449);
    assert_eq!(hill.part2(), 443);
}
