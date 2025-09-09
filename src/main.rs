use std::io::stdin;
use std::process::Command;
use std::io::stdout;
use std::io::Write;

fn main() {
    loop {
        print!("> ");
        stdout().flush();
        
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let mut child = Command::new(command).spawn().unwrap();

        child.wait();
    }
}
