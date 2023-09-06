#[derive(Debug, Clone, PartialEq)]
enum Node {
    File(File),
    Directory(Directory),
}

#[derive(Debug, Clone, PartialEq)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn build(output_line: &str) -> Option<Self> {
        let mut output_iter = output_line.split(' ');
        let filesize: u32;
        match output_iter.next() {
            Some(first) => match first {
                "dir" => return None,
                size => filesize = size.parse().expect("Invalid filesize: {size}"),
            },
            None => return None,
        }
        Some(Self {
            name: output_iter.next().expect("No filename given").to_string(),
            size: filesize,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Command {
    name: String,
    argument: String,
    output: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct Directory {
    name: String,
    contents: Vec<Node>,
}

impl Directory {
    fn build(name: String, command_list: &[Command]) -> (Self, usize) {
        let mut final_directory = Directory {
            name,
            contents: vec![],
        };
        let mut index = 0;
        let mut to_skip = 0;
        let mut command_iter = command_list.iter();
        loop {
            for _ in 0..to_skip {
                command_iter.next();
                index += 1;
            }
            let command = match command_iter.next() {
                Some(c) => c,
                None => break,
            };
            match command.name.as_str() {
                "cd" => match command.argument.as_str() {
                    ".." => return (final_directory, index + 1),
                    subdir => {
                        let (subdir, skip) =
                            Directory::build(subdir.to_string(), &command_list[index + 1..]);
                        final_directory.contents.push(Node::Directory(subdir));
                        to_skip = skip;
                    }
                },
                "ls" => {
                    // we can ignore empty directories -> only check for files in output
                    for output in command.output.iter() {
                        if let Some(file) = File::build(output) {
                            final_directory.contents.push(Node::File(file));
                        }
                    }
                }
                c => panic!("Unknown command: {c}"),
            }
            index += 1;
        }
        (final_directory, index)
    }

    fn size(&self) -> u32 {
        let mut current_size = 0;
        for node in &self.contents {
            match node {
                Node::File(file) => current_size += file.size,
                Node::Directory(dir) => current_size += dir.size(),
            }
        }
        current_size
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    let mut result = vec![];
    let mut commands = input.split('$');
    commands.next().expect("No commands");
    for command in commands {
        let mut command_output = vec![];
        let mut command_lines_iter = command.trim().lines();
        let mut command_iter = command_lines_iter
            .next()
            .expect("Command has no lines")
            .split(' ');
        let command_name = command_iter
            .next()
            .expect("Missing command name")
            .to_string();
        let mut command_arg = "".to_string();
        if let Some(arg) = command_iter.next() {
            command_arg = arg.to_string();
        }
        for line in command_lines_iter {
            command_output.push(line.to_string());
        }

        result.push(Command {
            name: command_name,
            argument: command_arg,
            output: command_output,
        });
    }
    result
}

fn find_all_sizes(dir: Directory) -> Vec<u32> {
    let mut sizes = vec![];
    for node in dir.contents {
        if let Node::Directory(dir) = node {
            sizes.push(dir.size());
            sizes.append(&mut find_all_sizes(dir).clone())
        }
    }
    sizes
}

fn main() {
    let input: Vec<Command> = parse_input(include_str!("../input.txt"))
        .into_iter()
        .skip(1)
        .collect();
    let dir_tree = Directory::build("/".to_string(), &input).0;
    let all_sizes = find_all_sizes(dir_tree);
    let mut total_of_target_dirs = 0;
    for size in all_sizes {
        if size <= 100000 {
            total_of_target_dirs += size;
        }
    }
    println!("Part 1: {total_of_target_dirs}");
}
