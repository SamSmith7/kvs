#[macro_use]
extern crate clap;
extern crate kvs;

use clap::App;
use std::process;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let success: Result<&str, &str> = match matches.subcommand() {
        ("get", Some(_matches)) => Err("unimplemented"),
        ("rm", Some(_matches)) => Err("unimplemented"),
        ("set", Some(_matches)) => Err("unimplemented"),
        _ => Err("Invalid actions"),
    };

    match success {
        Ok(_) => process::exit(0),
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };
}
