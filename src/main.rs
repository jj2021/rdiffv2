mod rdiff; // includes code from rdiff.rs 
use rdiff::run; // creates "named import" of the run function from the rdiff.rs module
use std::fs;
use std::env;

fn main() {

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("rdiff requires two file paths as arguments\n");
    return
  }

  let f1 = fs::read_to_string(&args[1]).expect("Could not read file");
  println!("file1 content:\n{}", f1);

  let f2 = fs::read_to_string(&args[2]).expect("Could not read file");
  println!("file2 content:\n{}", f2);

  run();
}
