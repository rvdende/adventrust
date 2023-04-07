#[derive(Debug)]

enum Type {
    ChangeDir,
    ListDir,
}

pub struct Command {
    cmd: Type,
}

impl Command {
    pub fn parse(input: String) -> Self {
        let mut cmd = Type::ListDir;

        let lines = input.lines().enumerate().for_each(|(row, line)| {
            if (row == 0) {
                println!("$ {}", line);

                cmd = match &line[0..2] {
                    "cd" => Type::ChangeDir,
                    _ => Type::ListDir,
                };
            }

            cmd = Type::ChangeDir;
        });

        Self { cmd }
    }
}

pub fn parse_command() {}
