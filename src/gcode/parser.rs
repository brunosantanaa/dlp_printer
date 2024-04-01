use super::gtypes::{Axes, Commands, Subcommands};
use super::file_mng::handler::open_gcode;

pub fn parse(input: &str) -> Vec<Commands> {
    let mut commands = Vec::new();

    for line in clean_comments(input).lines() {
        if let Some(command) = line.split_whitespace().nth(0) {
            match command.chars().nth(0) {
                Some('G') => commands.push(parse_g(line)),
                Some('M') => commands.push(parse_m(line)),
                _ => {}
            }
        }
    }
    commands
}

pub fn parse_from_file(path: &str) ->Vec<Commands> {
    let mut ret = Vec::<Commands>::new();
    if let Some(code) = open_gcode(path) {
        ret = parse(&code);
    }
    return ret;
}
fn parse_g(line: &str) -> Commands {
    let mut code = Commands::G0 {
        axes: None,
        subcommands: None,
    };

    if let Some(command) = line.split_whitespace().nth(0) {
        match command[1..].parse::<u16>().ok() {
            Some(0) => code = parse_g0(line), // G00
            Some(1) => code = parse_g1(line), // G01
            Some(4) => code = parse_g4(line), // G04
            _ => {}
        }
    }

    code
}

fn parse_m(line: &str) -> Commands {
    if let Some(command) = line.split_whitespace().nth(0) {
        match command[1..].parse::<u16>().ok() {
            Some(106) => return parse_m106(line), // M106
            Some(6054) => return parse_m6054(line),// M6054
            _ => {}
        }
    }

    Commands::M106 {pwm: Subcommands::S(0.0)}
}

fn parse_g0(line: &str) -> Commands {
    Commands::G0 {
        axes: Some(parse_axes(line)),
        subcommands: Some(parse_subcommands(line)),
    }
}

fn parse_g1(line: &str) -> Commands {
    Commands::G1 {
        axes: Some(parse_axes(line)),
        subcommands: Some(parse_subcommands(line)),
    }
}

fn parse_g4(line: &str) -> Commands {
    if let Some(subcommands) = Some(parse_subcommands(line)) {
        if let Some(cmd) = subcommands.first().cloned() {
            return Commands::G4 { time: cmd };
        }
    }

    Commands::G4 {time: Subcommands::P(0.0)}
}

fn parse_m106(line: &str) -> Commands {
    if let Some(subcommands) = Some(parse_subcommands(line)) {
        if let Some(cmd) = subcommands.first().cloned() {
            return Commands::M106 { pwm: cmd };
        }
    }

    Commands::M106 { pwm: Subcommands::S(0.0) }
}

fn parse_m6054(line: &str) -> Commands {
    if let Some(f_path) = line.split_whitespace().nth(1) {
        let mut path_str =String::from(f_path);
        path_str = path_str.replace("\"", "");
        return Commands::M6054 { path: path_str};
    }
    
    Commands::M6054 { path: String::from("") }
}

fn parse_axes(line: &str) -> Axes {
    let mut axes = Axes {
        x: None,
        y: None,
        z: None,
    };
    for token in line.split_whitespace() {
        match token.chars().nth(0) {
            Some('X') => axes.x = token[1..].parse::<f32>().ok(),
            Some('Y') => axes.y = token[1..].parse::<f32>().ok(),
            Some('Z') => axes.z = token[1..].parse::<f32>().ok(),
            _ => {}
        }
    }

    axes
}

fn parse_subcommands(line: &str) -> Vec<Subcommands> {
    let mut subcommands = Vec::new();
    for token in line.split_whitespace() {
        match token.chars().nth(0) {
            Some('E') => subcommands.push(Subcommands::E(
                token[1..].parse::<f32>().ok().unwrap_or_default(),
            )),
            Some('F') => subcommands.push(Subcommands::F(
                token[1..].parse::<f32>().ok().unwrap_or_default(),
            )),
            Some('S') => subcommands.push(Subcommands::S(
                token[1..].parse::<f32>().ok().unwrap_or_default(),
            )),
            Some('P') => subcommands.push(Subcommands::P(
                token[1..].parse::<f32>().ok().unwrap_or_default(),
            )),
            _ => {}
        }
    }

    subcommands
}

fn clean_comments(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            if let Some(index) = line.find(';') {
                &line[..index]
            } else {
                line
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}
