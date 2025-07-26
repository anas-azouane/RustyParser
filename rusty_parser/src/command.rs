use crate::parser::Element;
use std::process::Command;

pub fn elements_to_args(elements: &[Element]) -> Vec<String> {
    elements.iter().map(|el| el.name.clone()).collect()
}


pub fn run_cli_command(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("No command to run.");
        return;
    }

    let (cmd, cmd_args) = args.split_first().unwrap();

    let status = Command::new(cmd)
        .args(cmd_args)
        .status()
        .expect("Failed to run command");

    if !status.success() {
        eprintln!("Command exited with status: {:?}", status.code());
    }
}


