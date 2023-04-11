use colored::Colorize;

#[derive(Clone, Debug)]
struct Hill {
    map: Vec<MapPoint>,
    /** east-west size */
    width: usize,
    /** north-south size */
    height: usize,
}

#[derive(Clone, Copy, Debug)]
struct MapPoint {
    symbol: char,
    elevation: u8,
    is_start: bool,
    is_end: bool,
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

        let map: Vec<MapPoint> = data
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| MapPoint {
                    symbol: c,
                    elevation: char_to_elevation(c),
                    is_start: c == 'S',
                    is_end: c == 'E',
                })
            })
            .collect();

        Hill { map, width, height }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;

                let loc = &self.map[index];

                let elevation = loc.elevation;

                if loc.is_start {
                    print!("{}", loc.symbol.to_string().bright_yellow());
                }

                if loc.is_end {
                    print!("{}", loc.symbol.to_string().bright_green());
                }

                if !loc.is_start && !loc.is_end {
                    print!("{}", loc.symbol.to_string().truecolor(100, 100, 100));
                }

                // print!("{}", elevation)
            }
            println!();
        }
    }

    fn get_map_at_coord(&self, coord: &Coord) -> Option<MapPoint> {
        let index: usize = coord.y * self.width + coord.x;
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
            if f.elevation == current_map.elevation || f.elevation == current_map.elevation + 1 {
                possible.push(c);
            }
        }
    }

    /** get possible lower elevation steps */
    fn get_possible_steps(&self, prev: &Coord, coord: &Coord, depth: usize) -> usize {
        let current_map = self.get_map_at_coord(coord).unwrap();

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

        let test: Vec<Coord> = possible
            .iter()
            .filter(|f| {
                let pass = prev.x == f.x && prev.y == f.y;
                return !pass;
            })
            .map(|p| p.clone())
            .collect();

        if coord.x == self.get_end().x && coord.y == self.get_end().y {
            return depth;
        }

        if depth > 2000 {
            return 0;
        }

        let results: Vec<usize> = test
            .iter()
            .map(|c| self.get_possible_steps(coord, c, depth + 1))
            .filter(|i| *i != 0)
            .collect();

        if results.len() > 0 {
            return results.iter().min().unwrap().clone();
        }

        return 0;
    }

    fn get_end(&self) -> Coord {
        self.index_to_xy(self.map.iter().position(|i| i.symbol == 'E').unwrap())
    }

    fn find_shortest_path(&self) -> usize {
        let map = self.map.clone();

        let mut player = self.index_to_xy(map.iter().position(|i| i.symbol == 'S').unwrap());
        let end = self.index_to_xy(map.iter().position(|i| i.symbol == 'E').unwrap());
        println!("player {},{}", player.x, player.y);
        println!("end {},{}", end.x, end.y);

        let mut stop = false;

        let steps = self.get_possible_steps(&player, &player, 0);
        println!("steps {}", steps);
        return steps;
    }
}

pub fn run() {
    let hill = Hill::load("src/advent2022/day12/sample.txt");
    hill.print();

    hill.find_shortest_path();
}

#[test]
fn sample() {
    assert_eq!(
        Hill::load("src/advent2022/day12/sample.txt").find_shortest_path(),
        31
    );
}
