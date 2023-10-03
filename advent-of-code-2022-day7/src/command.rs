use regex::Regex;
const CD_COMMAND: &str = "cd";
const LS_COMMAND: &str = "ls";
pub const COMMAND_STARTING_CHARACTER: char = '$';


pub enum Command {
    Cd(String),
    GoBack,
    Ls,
    Unknown
}

pub fn handle_command(line: &str) -> Command {
    if line.contains(CD_COMMAND) {
        return handle_cd_command(line).unwrap();
    }
    if line.contains(LS_COMMAND) {
        return Command::Ls;
    }
    Command::Unknown
}

fn handle_cd_command(line: &str) -> Option<Command> {
    let re = Regex::new(r"\$ cd (?<path>.+)").unwrap();
    let Some(caps) = re.captures(line) else {
        println!("Couldn't get CD path");
        return None;
    };
    let path = &caps["path"];
    println!("cd to {}", path);
    if path == ".." {
        return Some(Command::GoBack);
    }
    Some(Command::Cd(String::from(path)))
}
