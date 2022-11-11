use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Stdin;
use std::io::StdinLock;
use std::io::prelude::*;
use regex::Regex;
use clap::{ App, Arg, ArgMatches };

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line: String = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args: ArgMatches = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true)
        )
        .arg(Arg::with_name("input").help("File to search").takes_value(true).required(true))
        .get_matches();

    let pattern: &str = args.value_of("pattern").unwrap();
    let re: Regex = Regex::new(pattern).unwrap();

    let input: &str = args.value_of("input").unwrap_or("-");
    if input == "-" {
        let stdin: Stdin = io::stdin();
        let reader: StdinLock = stdin.lock();
        process_lines(reader, re);
    } else {
        let f: File = File::open(input).unwrap();
        let reader: BufReader<File> = BufReader::new(f);
        process_lines(reader, re)
    }
}