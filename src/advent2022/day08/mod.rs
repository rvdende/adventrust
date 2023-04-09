struct Forest {
    trees: Vec<isize>,
    width: isize,
    height: isize,
}

enum Direction {
    North,
    East,
    South,
    West,
}

struct Coord {
    x: isize,
    y: isize,
}

impl Forest {
    fn load(filename: &str) -> Self {
        let data = std::fs::read_to_string(filename).unwrap();

        let width: isize = data.lines().next().unwrap().len() as isize;
        let height: isize = data.lines().count() as isize;

        let trees: Vec<isize> = data
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_string().parse::<isize>().unwrap())
            .collect();

        Self {
            width,
            height,
            trees,
        }
    }

    /** how many trees are visible from outside the grid? */
    #[allow(dead_code)]
    fn part1(&self) -> isize {
        let mut total_visible = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let visible = self.check_tree_visibility(x, y);
                if visible {
                    total_visible += 1;
                }
            }
        }

        total_visible
    }

    fn get_height(&self, x: isize, y: isize) -> isize {
        if self.out_of_bounds(x, y) {
            unreachable!(
                "x y out of bounds! {} {} [{} {}]",
                x, y, self.width, self.height
            );
        }

        let index = y * self.width + x;
        self.trees[index as usize]
    }

    fn out_of_bounds(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x > (self.width - 1) || y > (self.height - 1)
    }

    #[allow(dead_code)]
    fn check_tree_visibility(&self, x: isize, y: isize) -> bool {
        if x == 0 {
            return true;
        }
        if y == 0 {
            return true;
        }
        if x == self.width - 1 {
            return true;
        }
        if y == self.height - 1 {
            return true;
        }

        let tree_height = self.get_height(x, y);

        let mut visible_west = true;
        (0..x).for_each(|col| {
            if self.get_height(col, y) >= tree_height {
                visible_west = false;
            }
        });

        let mut visible_east = true;
        ((x + 1)..self.width).for_each(|col| {
            if self.get_height(col, y) >= tree_height {
                visible_east = false;
            }
        });

        let mut visible_north = true;
        (0..y).for_each(|row| {
            if self.get_height(x, row) >= tree_height {
                visible_north = false;
            }
        });

        let mut visible_south = true;
        ((y + 1)..self.height).for_each(|row| {
            if self.get_height(x, row) >= tree_height {
                visible_south = false;
            }
        });

        visible_west || visible_east || visible_north || visible_south
    }

    fn walk(&self, x: isize, y: isize, direction: &Direction) -> isize {
        let mut next: Coord = Coord {
            x: x as isize,
            y: y as isize,
        };

        let mut score = 0;
        let tree_height = self.get_height(x, y);

        let mut blocked = false;

        while !blocked {
            next = match direction {
                Direction::North => Coord {
                    x: next.x,
                    y: next.y - 1,
                },
                Direction::East => Coord {
                    x: next.x + 1,
                    y: next.y,
                },
                Direction::South => Coord {
                    x: next.x,
                    y: next.y + 1,
                },
                Direction::West => Coord {
                    x: next.x - 1,
                    y: next.y,
                },
            };

            if self.out_of_bounds(next.x, next.y) {
                blocked = true
            } else {
                score += 1;
                if self.get_height(next.x, next.y) >= tree_height {
                    blocked = true;
                }
            }
        }

        return score;
    }

    fn scenic_score_for_tree(&self, x: isize, y: isize) -> isize {
        let mut total_score = 1;

        for dir in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        {
            total_score *= self.walk(x, y, dir);
        }

        return total_score;
    }

    fn tree_coord_from_id(&self, id: isize) -> Coord {
        let x = id % self.width;
        let y = id / self.width;
        Coord { x, y }
    }

    /** What is the highest scenic score possible for any tree? */
    fn part2(&self) -> isize {
        self.trees
            .iter()
            .enumerate()
            .map(|(id, _tree)| {
                let coord = self.tree_coord_from_id(id as isize);

                self.scenic_score_for_tree(coord.x, coord.y)
            })
            .max()
            .unwrap()
    }
}

#[allow(dead_code)]
pub fn run() {
    let forest = Forest::load("src/advent2022/day08/input.txt");
    let maxscore = forest.part2();
    println!("max score: {}", maxscore);
}

#[test]
fn sample() {
    let forest = Forest::load("src/advent2022/day08/sample.txt");
    assert_eq!(forest.scenic_score_for_tree(2, 1), 4);
    assert_eq!(forest.scenic_score_for_tree(2, 3), 8);
    assert_eq!(forest.part1(), 21);
    assert_eq!(forest.part2(), 8);
}

#[test]
fn input() {
    let forest = Forest::load("src/advent2022/day08/input.txt");
    assert_eq!(forest.part1(), 1538);
    assert_eq!(forest.part2(), 496125);
}
