use uuid::{self, Uuid};

#[derive(Debug)]
struct FileData {
    id: Uuid,
    name: String,
    size: u64,
    is_dir: bool,
    parent_id: Uuid,
}

struct Computer {
    fs_current: Uuid,
    fs_root: Uuid,
    fs: Vec<FileData>,
}

impl Computer {
    fn new() -> Self {
        let mut fs: Vec<FileData> = Vec::new();

        let current_id = Uuid::new_v4();

        fs.push(FileData {
            id: current_id.clone(),
            name: "".to_string(),
            size: 0,
            is_dir: true,
            parent_id: current_id.clone(),
        });

        Self {
            fs,
            fs_root: current_id.clone(),
            fs_current: current_id,
        }
    }

    fn get_size(&self, id: Uuid) -> u64 {
        let mut size = 0;

        self.fs.iter().for_each(|f| {
            if f.parent_id == id && f.id != f.parent_id {
                if f.is_dir {
                    size += self.get_size(f.id);
                } else {
                    size += f.size;
                }
            }
        });

        size
    }

    fn get_size_by_name(&self, name: &str) -> u64 {
        let f = self.fs.iter().find(|f| f.name == name).unwrap();
        self.get_size(f.id)
    }

    fn get_by_max_size(&self, maxsize: u64) -> Vec<&FileData> {
        let mut result: Vec<&FileData> = Vec::new();

        self.fs.iter().for_each(|f| {
            if f.is_dir {
                let size = self.get_size(f.id);
                if size <= maxsize {
                    result.push(f.clone());
                }
            }
        });

        result
    }

    fn cd(&mut self, name: &str) {
        if name == "/" {
            self.fs_current = self.fs_root.clone();
            return;
        }

        if name == ".." {
            let f = self.fs.iter().find(|f| f.id == self.fs_current).unwrap();

            let p = self.fs.iter().find(|f| f.id == f.parent_id).unwrap();
            self.fs_current = p.id.clone();
            return;
        }

        let f = self.fs.iter().find(|f| f.name == name).unwrap();
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
                                size: filesize.parse::<u64>().unwrap(),
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

    fn part1(&self) -> u64 {
        let list = self.get_by_max_size(100_000);

        let total: u64 = list
            .iter()
            .map(|f| {
                let size = self.get_size(f.id);
                println!("part1: {} {}", f.name, size);

                size
            })
            .sum();

        total
    }
}

pub fn run() {
    let mut samplepc = Computer::new();
    samplepc.init("src/advent2022/day07/sample.txt");
    samplepc.print();
    let total = samplepc.get_size(samplepc.fs_root);
    println!("total: {}", total);

    let test_e = samplepc.get_size_by_name("e");
    println!("test_e: {}", test_e);
}

#[test]
fn sample_part_1() {
    let mut samplepc = Computer::new();
    samplepc.init("src/advent2022/day07/sample.txt");

    assert_eq!(samplepc.get_size_by_name("e"), 584);
    assert_eq!(samplepc.get_size_by_name("a"), 94853);
    assert_eq!(samplepc.get_size_by_name("d"), 24933642);
    assert_eq!(samplepc.get_size(samplepc.fs_root), 48381165);

    assert_eq!(samplepc.part1(), 95437);
}

#[test]
fn input_part_1() {}
