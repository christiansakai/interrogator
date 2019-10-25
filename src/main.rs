use std::env;
use std::process;
use std::process::Command;

use interrogator; 

fn main() {
    let argv: Vec<String> = env::args().collect();
    let config = Config::new(argv);

    loop {
        let question = match config.mode {
            Mode::SDE => interrogator::pick_rand_sde(), 
            Mode::PM => interrogator::pick_rand_pm(), 
            Mode::COMMON => interrogator::pick_rand_common(), 
            Mode::ALL => interrogator::pick_rand_all(), 
        };

        let mut child = Command::new("say")
            .arg(question)
            .spawn()
            .expect("Failed to execute process");

        let _ = child.wait();
    }
}

struct Config {
    mode: Mode,
}

enum Mode {
    SDE,
    PM,
    COMMON,
    ALL,
}

impl Config {
    fn new(argv: Vec<String>) -> Config {
        if argv.len() < 2 {
            eprintln!("Please provide MODE");
            process::exit(1);
        }

        let mode = match argv[1].as_ref() {
            "sde" => Mode::SDE,
            "pm" => Mode::PM,
            "common" => Mode::COMMON,
            "all" => Mode::ALL,
            _ => {
                eprintln!("Modes are: sde, pm, common, all");
                process::exit(1);
            },
        };

        Config { mode }
    }
}
