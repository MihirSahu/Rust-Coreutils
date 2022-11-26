#![allow(non_snake_case)]

use clap::Parser;
use std::path::Path;
use std::fs;
use std::process;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "grep")]
#[command(author = "Mihir Sahu <2002mihir@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "grep rewritten with Rust", long_about = None)]
struct Cli {

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    insensitive: Option<bool>,

    #[arg(short, long)]
    context: Option<usize>,

    pattern: Option<String>,

    path: Option<std::path::PathBuf>,

}

fn main() {

    let args = Cli::parse();
    let context;

    if args.context != None {
        context = args.context.unwrap();
    }
    else {
        context = 5;
    }

    if (args.pattern != None) && (args.path != None) {

        if !Path::new(args.path.as_ref().unwrap()).exists() {
            println!("File not found.");
            process::exit(1);
        }

        // https://jakedawkins.com/2020-04-16-unwrap-expect-rust/
        let contents = fs::read_to_string(args.path.unwrap());

        if args.insensitive.unwrap() == true {
            match contents.as_ref().unwrap().to_lowercase().find(&args.pattern.as_ref().unwrap().to_lowercase()) {
                Some(index) => println!("{}{}{}", 
                    &contents.as_ref().unwrap()[index - context..index], 
                    &contents.as_ref().unwrap()[index..index + args.pattern.as_ref().unwrap().len()].blue(), 
                    &contents.as_ref().unwrap()[index + args.pattern.as_ref().unwrap().len()..index + args.pattern.unwrap().len() + context]),
                None => println!("Could not find pattern"),
            }
        }
        else {
            match contents.as_ref().unwrap().find(args.pattern.as_ref().unwrap()) {
                Some(index) => println!("{}{}{}", 
                    &contents.as_ref().unwrap()[index - context..index], 
                    &contents.as_ref().unwrap()[index..index + args.pattern.as_ref().unwrap().len()].blue(), 
                    &contents.as_ref().unwrap()[index + args.pattern.as_ref().unwrap().len()..index + args.pattern.unwrap().len() + context]),
                None => println!("Could not find pattern"),
            }
        }
    }
}
