#![allow(non_snake_case)]

use clap::Parser;
use std::fs;
use std::os::linux::fs::MetadataExt;
use colored::Colorize;

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

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    long: Option<bool>,
}


fn main() {

    let args = Cli::parse();

    let mut paths = fs::read_dir(".").unwrap();

    if args.path != None {
        paths = fs::read_dir(args.path.unwrap()).unwrap();
    }

    if args.long.unwrap() == true {
        println!("{0: <25}   {1: ^10}   {2: ^10}   {3: ^10}  \n", "Name", "UID", "GID", "Size");
    }

    for path in paths {

        if path.as_ref().unwrap().path().is_file() {

            let file_name: String = path.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap().to_string();

            if (file_name.chars().nth(0).unwrap() == '.') && (args.all.unwrap() == false) {
                continue;
            }
            else if args.long.unwrap() == true {
                // println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 0, 0, 0, 0);
                // https://doc.rust-lang.org/std/fmt/
                println!(
                    "{0: <25} | {1: ^10} | {2: ^10} | {3: ^10} |", 
                    file_name, 
                    fs::metadata(path.as_ref().unwrap().path()).ok().unwrap().st_uid(), 
                    fs::metadata(path.as_ref().unwrap().path()).ok().unwrap().st_gid(),
                    fs::metadata(path.as_ref().unwrap().path()).ok().unwrap().st_size(),
                );
            }
            else {
                println!("{}", file_name);
            }
        }
        else if path.as_ref().unwrap().path().is_dir() {

            let mut dir_name: String = path.as_ref().unwrap().path().file_name().unwrap().to_str().unwrap().to_string();
            dir_name.push('/');

            if (dir_name.chars().nth(0).unwrap() == '.') && (args.all.unwrap() == false) {
                continue;
            }
            else if args.long.unwrap() == true {
                println!(
                    "{0: <25} | {1: ^10} | {2: ^10} | {3: ^10} |", 
                    dir_name.blue(), 
                    fs::metadata(path.as_ref().unwrap().path()).ok().unwrap().st_uid(), 
                    fs::metadata(path.as_ref().unwrap().path()).ok().unwrap().st_gid(),
                    fs::metadata(path.as_ref().unwrap().path()).ok().unwrap().st_size(),
                );
            }
            else {
                println!("{}", dir_name.blue());
            }
        }
    }
}
