use std::collections::VecDeque;
use std::process::Command;

pub fn string_to_command(command: &str) -> Command {
    let mut split: VecDeque<String> = command.split_whitespace().map(String::from).collect();
    let mut command = Command::new(split.get(0).unwrap());
    split.pop_front();

    for value in split {
        command.arg(value);
    }

    command
}