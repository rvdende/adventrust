// https://docs.rs/pathfinding/latest/pathfinding/
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, hill: &Hill) -> Vec<Pos> {
        let coord = Coord {
            x: self.0.clone() as usize,
            y: self.1.clone() as usize,
        };

        let possible = hill.get_possible_steps(&coord);

        let options = possible
            .iter()
            .map(|p| Pos(p.x as i32, p.y as i32))
            .collect();

        return options;
    }
}

#[derive(Clone, Debug)]
struct Hill {
    map: Vec<MapPoint>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug)]
struct MapPoint {
    symbol: char,
    elevation: u8,

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

    return val - ('a' as u8);
}

impl Hill {
    fn load(path: &str) -> Hill {
        let data = std::fs::read_to_string(path).unwrap();
        let width = data.lines().next().unwrap().len();
        let height: usize = data.lines().count();
        let wi32 = width as i32;

        let mut map: Vec<MapPoint> = data
            .lines()
            .flat_map(|l| {
                l.chars().map(move |c| {
                    return MapPoint {
                        symbol: c,
                        elevation: char_to_elevation(c),
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

        Hill { map, width, height }
    }

    fn get_map_at_coord(&self, coord: &Coord) -> Option<MapPoint> {
        let index: usize = coord.y * self.width + coord.x;

        if index > self.map.len() - 1 {
            return None;
        }

        return Some(self.map[index]);
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
        if x > (self.width as isize) {
            return;
        }
        if y > (self.height as isize) {
            return;
        }

        let c = Coord {
            x: x as usize,
            y: y as usize,
        };

        if let Some(f) = self.get_map_at_coord(&c) {
            if f.elevation <= current_map.elevation + 1 {
                possible.push(c);
            }
        }
    }

    /** get possible lower elevation steps */
    fn get_possible_steps(&self, coord: &Coord) -> Vec<Coord> {
        let current_map = match self.get_map_at_coord(coord) {
            Some(x) => x,
            None => return Vec::new(),
        };

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

        possible
    }

    fn get_start(&self) -> Coord {
        self.index_to_xy(self.map.iter().position(|i| i.symbol == 'S').unwrap())
    }

    fn get_end(&self) -> Coord {
        self.index_to_xy(self.map.iter().position(|i| i.symbol == 'E').unwrap())
    }

    fn part1(&self) -> usize {
        let goal = Pos(self.get_end().x as i32, self.get_end().y as i32);
        let start = Pos(self.get_start().x as i32, self.get_start().y as i32);

        let result = bfs(&start, |p| p.successors(self), |p| *p == goal);

        let steps = match result {
            Some(x) => x.iter().count(),
            None => 99999999,
        };

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
                let goal = Pos(self.get_end().x as i32, self.get_end().y as i32);
                let start = Pos(s.x as i32, s.y as i32);

                let result = bfs(&start, |p| p.successors(self), |p| *p == goal);

                let steps = match result {
                    Some(x) => x.iter().count(),
                    None => 9999999,
                } - 1;

                if steps < lowest {
                    lowest = steps;
                }

                return steps;
            })
            .collect();

        let answer = shortest.iter().min().unwrap();

        return answer.clone();
    }
}

#[allow(dead_code)]
pub fn run() {
    let hillsample = Hill::load("src/advent2022/day12/sample.txt");

    let result = hillsample.part1();
    println!("hillsample answer: {:?}", result);

    let resultp2 = hillsample.part2();
    println!("hillsample answer: {:?}", resultp2);

    let hill = Hill::load("src/advent2022/day12/input.txt");

    let result = hill.part1();
    println!("answer: {:?}", result);

    let resultp2 = hill.part2();
    println!("answerp2: {:?}", resultp2);
}

#[test]
fn sample() {
    let hill = Hill::load("src/advent2022/day12/sample.txt");
    assert_eq!(hill.part1(), 31);
    assert_eq!(hill.part2(), 29);
}

#[test]
fn input() {
    let hill = Hill::load("src/advent2022/day12/input.txt");
    assert_eq!(hill.part1(), 449);
    assert_eq!(hill.part2(), 443);
}
