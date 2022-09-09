use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// R&W files from cli
#[derive(Parser, Debug)]
#[clap(author,about, long_about = None)]
struct Cli {
    /// Name of the in file
    in_file: String,
    /// Name of the out file
    out_file: String,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    let file = File::open(&args.in_file).expect("file not found!");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        println!("{}", line.unwrap());
    }
    // let mut line_nums = String::new();
    // let temp: u32;
    // match std::io::stdin().read_line(&mut line_nums) {
    //     Ok(_) => {
    //         temp = line_nums.trim().parse::<u32>().expect("Not valid input");
    //     }
    //     Err(error) => {
    //         panic!("{error}");
    //     }
    // };
    // // let num:u32 = match line_nums.trim().parse::<u32>() {
    // //     Ok(i) => i,    //     Err(_) => panic!("Not a valid input"),
    // //     Err(_) => panic!("Not a valid input"),
    // // };
    // println!("The num is  {temp}");
}
