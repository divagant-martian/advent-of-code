use crate::walk_item::{Command, Response, WalkItem};

pub fn parse(input: &str) -> impl Iterator<Item = WalkItem> + '_ {
    input.lines().map(|l| {
        if let Some(command) = l.strip_prefix("$ ") {
            let command = if let Some(dir) = command.strip_prefix("cd ") {
                Command::Cd(dir)
            } else {
                assert_eq!(command, "ls");
                Command::Ls
            };
            WalkItem::Command(command)
        } else {
            let response = if let Some(name) = l.strip_prefix("dir ") {
                Response::Dir { name }
            } else {
                let (size_str, name) = l.split_once(' ').unwrap();
                let size: usize = size_str.parse().unwrap();
                Response::File { name, size }
            };
            WalkItem::Response(response)
        }
    })
}
