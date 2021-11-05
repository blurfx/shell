use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut args = input.trim().split_whitespace();
        let command = args.next().unwrap();
        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();
        child.wait();
    }
}
