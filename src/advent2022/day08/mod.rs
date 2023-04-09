struct Forest {
    trees: Vec<usize>,
    width: usize,
    height: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Forest {
    fn load(filename: &str) -> Self {
        let data = std::fs::read_to_string(filename).unwrap();

        let width = data.lines().next().unwrap().len();
        let height = data.lines().count();

        let trees: Vec<usize> = data
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();

        Self {
            width,
            height,
            trees,
        }
    }

    fn print(&self) {
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("trees: {:?}", self.trees);
    }

    fn get_height(&self, x: usize, y: usize) -> usize {
        let index = y * self.width + x;
        self.trees[index]
    }

    fn check_tree_visibility(&self, x: usize, y: usize) -> bool {
        let mut visible = false;

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

        let treeHeight = self.get_height(x, y);

        // INVERT LOGIC
        // check if the tree is hidden in all directions.. must be hidden in all 4.
        // to left

        // to left
        for col in 0..(x - 1) {
            if self.get_height(col, y) <= treeHeight {
                return true;
            }
        }

        // to right
        for col in (x + 1)..self.width {
            if self.get_height(col, y) >= treeHeight {
                return true;
            }
        }

        // to top

        for row in 0..(y - 1) {
            if self.get_height(x, row) >= treeHeight {
                return true;
            }
        }

        // to bottom

        for row in (y + 1)..self.height {
            if self.get_height(x, row) >= treeHeight {
                return true;
            }
        }

        return false;
    }
}

pub fn run() {
    let forest = Forest::load("src/advent2022/day08/sample.txt");
    forest.print();
    // Forest::load("src/advent2022/day08/input.txt");
}
