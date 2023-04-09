#[derive(Debug)]
struct FileData {
    name: String,
    size: u64,
    is_dir: bool,
}

fn process(filename: &str) {
    let mut pwd = "/";
    let mut fs: Vec<FileData> = Vec::new();

    let commands: Vec<_> = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split("$")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut cmd = "";

            l.trim().lines().enumerate().for_each(|(row, line)| {
                if row == 0 {
                    cmd = &line[0..2];

                    if cmd == "cd" {
                        let folder = &line[3..];
                        println!("CD {}", folder);

                        if &folder[0..1] == "/" {
                            pwd = folder
                        }
                    }

                    if cmd == "ls" {
                        println!("LS [{}]", line);
                    }
                }

                if row > 0 {
                    println!("  {}", line);

                    if &line[0..3] == "dir" {
                        let dirname = &line[4..];
                        println!(" DIRNAME: [{}]", dirname)
                    } else {
                        let (filesize, filename) = line.split_once(" ").unwrap();

                        fs.push(FileData {
                            name: filename.to_string(),
                            size: filesize.parse::<u64>().unwrap(),
                            is_dir: false,
                        });
                    }
                }
            })
        })
        .collect();

    fs.iter().for_each(|f| {
        println!("FS: {:?}", f);
    })
}

pub fn run() {
    process("src/advent2022/day07/sample.txt");
}

#[test]
fn test() {}
