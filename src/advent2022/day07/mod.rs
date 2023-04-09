use std::str::FromStr;

use uuid::{self, Uuid};

#[derive(Debug)]
struct FileData {
    id: Uuid,
    name: String,
    size: u128,
    is_dir: bool,
    parent_id: Uuid,
}

struct Computer {
    fs_current: Uuid,
    fs_root: Uuid,
    fs: Vec<FileData>,
}

#[allow(dead_code)]
impl Computer {
    fn new() -> Self {
        let mut fs: Vec<FileData> = Vec::new();

        let current_id = Uuid::new_v4();

        fs.push(FileData {
            id: current_id.clone(),
            name: "".to_string(),
            size: 0,
            is_dir: true,
            parent_id: Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap(),
        });

        Self {
            fs,
            fs_root: current_id.clone(),
            fs_current: current_id,
        }
    }

    fn get_size(&self, id: Uuid, depth: u128) -> u128 {
        let mut size = 0;

        self.fs.iter().for_each(|f| {
            if f.parent_id == id {
                if f.is_dir {
                    println!("{}/{} {}", " ".repeat(depth as usize), f.name, f.size);
                    size += self.get_size(f.id, depth + 1);
                } else {
                    size += f.size;
                    println!("{} {} {}", " ".repeat(depth as usize), f.name, f.size);
                }
            }
        });

        size
    }

    fn get_size_by_name(&self, name: &str) -> u128 {
        let f = self.fs.iter().find(|f| f.name == name).unwrap();
        self.get_size(f.id, 0)
    }

    fn get_by_max_size(&self, maxsize: u128) -> Vec<&FileData> {
        let mut result: Vec<&FileData> = Vec::new();

        self.fs.iter().for_each(|f| {
            if f.is_dir {
                let size = self.get_size(f.id, 0);
                if size <= maxsize {
                    result.push(f.clone());
                }
            }
        });

        result
    }

    fn findbyid(&self, id: Uuid) -> &FileData {
        self.fs.iter().find(|f| f.id == id).unwrap()
    }

    fn cd(&mut self, name: &str) {
        if name == "/" {
            self.fs_current = self.fs_root.clone();
            return;
        }

        if name == ".." {
            self.fs_current = self.findbyid(self.fs_current).parent_id;
            return;
        }

        // find children..

        let list = self
            .fs
            .iter()
            .filter(|f| f.parent_id == self.fs_current)
            .collect::<Vec<_>>();

        let f = list.iter().find(|f| f.name == name).unwrap();
        self.fs_current = f.id.clone();
    }

    fn init(&mut self, filename: &str) {
        std::fs::read_to_string(filename)
            .unwrap()
            .trim()
            .split("$")
            .filter(|l| !l.is_empty())
            .for_each(|l| {
                let mut cmd = "";

                l.trim().lines().enumerate().for_each(|(row, line)| {
                    ////
                    if row == 0 {
                        cmd = &line[0..2];

                        if cmd == "cd" {
                            let folder = &line[3..];
                            self.cd(folder);
                        }
                    }
                    ////
                    if row > 0 {
                        if &line[0..3] == "dir" {
                            let dirname = &line[4..];

                            self.fs.push(FileData {
                                id: Uuid::new_v4(),
                                name: dirname.to_string(),
                                size: 0,
                                is_dir: true,
                                parent_id: self.fs_current.clone(),
                            })
                        } else {
                            let (filesize, filename) = line.split_once(" ").unwrap();

                            self.fs.push(FileData {
                                id: Uuid::new_v4(),
                                name: filename.to_string(),
                                size: filesize.parse::<u128>().unwrap(),
                                is_dir: false,
                                parent_id: self.fs_current.clone(),
                            });
                        }
                    }
                    ////
                })
            });
    }

    fn print(&self) {
        println!("---------------");
        self.fs.iter().for_each(|f| {
            if f.is_dir {
                println!("DIR /{}", f.name);
            } else {
                println!("FILE /{} {}", f.size, f.name);
            }
        })
    }

    fn part1(&self) -> u128 {
        let list = self.get_by_max_size(100_000);

        let total: u128 = list
            .iter()
            .map(|f| {
                let size = self.get_size(f.id, 0);
                println!("part1: {} {}", f.name, size);

                size
            })
            .sum();

        total
    }

    fn part2(&self) -> u128 {
        let total_disk_size = 70_000_000;
        let required_free_space = 30_000_000;

        let current_used_space = self.get_size(self.fs_root, 0);

        let current_free_space = total_disk_size - current_used_space;

        let minimum_to_delete = required_free_space - current_free_space;

        println!(
            "part2: {} {} {}",
            current_used_space, current_free_space, minimum_to_delete
        );

        let mut dirs: Vec<u128> = self
            .fs
            .iter()
            .filter(|f| f.is_dir)
            .map(|s| self.get_size(s.id, 0))
            .filter(|s| *s > minimum_to_delete)
            .collect();

        dirs.sort();

        println!("smallest dir to delete: {:?}", dirs[0]);

        return dirs[0];
    }
}

#[allow(dead_code)]
pub fn run() {
    let mut samplepc = Computer::new();
    samplepc.init("src/advent2022/day07/sample.txt");
    samplepc.part2();
}

#[test]
fn sample_part_1() {
    let mut samplepc = Computer::new();
    samplepc.init("src/advent2022/day07/sample.txt");

    assert_eq!(samplepc.get_size_by_name("e"), 584);
    assert_eq!(samplepc.get_size_by_name("a"), 94853);
    assert_eq!(samplepc.get_size_by_name("d"), 24933642);
    assert_eq!(samplepc.get_size(samplepc.fs_root, 0), 48381165);
    assert_eq!(samplepc.part1(), 95437);
}

#[test]
fn sample_part_2() {
    let mut samplepc = Computer::new();
    samplepc.init("src/advent2022/day07/sample.txt");

    assert_eq!(samplepc.part2(), 24933642);
}

#[test]
fn input_part_1() {
    let mut inputp1 = Computer::new();
    inputp1.init("src/advent2022/day07/input.txt");
    assert_eq!(inputp1.part1(), 1642503);
}

#[test]
fn input_part_2() {
    let mut inputp1 = Computer::new();
    inputp1.init("src/advent2022/day07/input.txt");
    assert_eq!(inputp1.part2(), 6999588);
}
