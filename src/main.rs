#[macro_use]
extern crate clap;
extern crate rand;

use std::io;
use std::io::{Write, BufRead};
use std::process;

use clap::App;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

fn exit(msg: &str) -> ! {
    let mut err = io::stderr();
    let _ = write!(err, "{}\n", msg);
    process::exit(1);
}

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let n = matches.value_of("n").unwrap();

    // validations on N
    //
    // n is an int
    let n: i32 = n.parse().unwrap_or_else(|_| {
        exit("N should be a valid integer");
    });
    // n > 0
    if n < 1 {
        exit("N should be strictly positive ");
    }

    let input = io::stdin();
    let handle = input.lock();

    let mut rng = rand::thread_rng();
    let between = Range::new(1, n + 1);

    let mut lines: Vec<_> = handle.lines()
        .map(|l| l.unwrap_or_else(|e| {
            let msg = format!("Failed to read line: {}", e);
            exit(&msg);
        }))
        .filter(|_| between.ind_sample(&mut rng) == 1)
        .take_while(|l| l != EOF)
        .collect();

    if matches.is_present("shuffle") {
        rng.shuffle(&mut lines);
    }

    for line in lines {
        println!("{}", line);
    }
}

const EOF: &'static str = "";
