#![allow(non_snake_case)]

//use std::env;
use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(name = "Rust-ls")]
#[command(author = "Mihir Sahu <2002mihir@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "ls rewritten with Rust", long_about = None)]
struct Cli {

    #[arg(short, long)]
    path: Option<std::path::PathBuf>,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    all: Option<bool>,
}


fn main() {

    let args = Cli::parse();

    let mut paths = fs::read_dir(".").unwrap();

    if args.path != None {
        paths = fs::read_dir(args.path.unwrap()).unwrap();
    }

    for path in paths {

        let file_name: String = path.unwrap().path().file_name().unwrap().to_str().unwrap().to_string();
        if (file_name.chars().nth(0).unwrap() == '.') && (args.all.unwrap() == false) {
            continue;
        }
        else {
            print!("{} \n", file_name);
        }
    }
}
