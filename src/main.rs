mod rdiff; // includes code from rdiff.rs 
use rdiff::run; // creates "named import" of the run function from the rdiff.rs module
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("rdiff requires two file paths as arguments\n");
    return
  }

  let f1 = fs::read_to_string(&args[1]).expect("Could not read file");
  let f2 = fs::read_to_string(&args[2]).expect("Could not read file");

  // Place lines of file into a vec
  if let Ok(f) = File::open(&args[1]) {
    let reader = io::BufReader::new(f);
    let num_lines = reader.lines().count();
    println!("file has {} lines\n", num_lines);
  }


  run(&f1, &f2);
}
