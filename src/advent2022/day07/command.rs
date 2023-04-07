#[derive(Debug, PartialEq)]
enum Type {
    None,
    ChangeDir(String),
    ListDir(Vec<String>),
}

#[derive(Debug)]
pub struct Command {
    cmd: Type,
}

impl Command {
    pub fn parse(input: String) -> Self {
        let mut command = Command { cmd: Type::None };

        let lines = input.lines().enumerate().for_each(|(row, line)| {
            if row == 0 {
                println!("$ {}", line);

                command.cmd = match &line[0..2] {
                    "cd" => Type::ChangeDir(line[3..].to_string()),
                    _ => Type::ListDir(vec![]),
                };
            }

            if row > 0 {
                if let Type::ListDir(x) = &command.cmd {
                    println!("  {}", line);
                    x.push(line.to_string());
                }
                println!("  {}", line);

                if &line[0..3] == "dir" {
                    let dirname = &line[4..];
                    println!(" DIRNAME: [{}]", dirname)
                }
            }
        });

        println!(" command: {:?}", command);

        command
    }
}

pub fn parse_command() {}
