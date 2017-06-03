#[macro_use]
extern crate clap;
extern crate rand;

use std::io;
use std::io::{
    Write,
    BufRead,
    BufReader,
    BufWriter
};
use std::process;
use std::fmt::Debug;

use clap::App;
use rand::distributions::{IndependentSample, Range};

fn unwrap_or_exit<T, E: Debug>(r: Result<T, E>, prefix: &str) -> T {
    r.unwrap_or_else(|e| {
        let msg = format!("{}: {:?}", prefix, e);
        exit(&msg);
    })
}

fn exit(msg: &str) -> ! {
    let mut err = io::stderr();
    writeln!(err, "{}", msg).unwrap();
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

    let mut rng = rand::thread_rng();
    let between = Range::new(1, n + 1);

    // read from input (lazy)

    let input = io::stdin();
    let reader = BufReader::new(input.lock());

    let lines = reader.lines()
        .map(|l| unwrap_or_exit(l, "Failed to read line"))
        .filter(|_| between.ind_sample(&mut rng) == 1)
        .take_while(|l| l != EOF);

    // write to output

    let output = io::stdout();
    let mut writer = BufWriter::new(output.lock());

    for line in lines {
        let write = writeln!(writer, "{}", line);
        unwrap_or_exit(write, "Failed to write line");
    }
}

const EOF: &'static str = "";
