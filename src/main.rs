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

  // convert files to strings
  let s1 = fs::read_to_string(&args[1]).expect("Could not read file");
  let s2 = fs::read_to_string(&args[2]).expect("Could not read file");

  // diff strings
  run(&s1, &s2);

  // Place lines of file into a vec
  let f3 = file_to_vec(&args[1]);
  let f4 = file_to_vec(&args[2]);

  // diff files
  rdiff::run_on_files(f3, f4);
}

fn file_to_vec(file_name: &str) -> Vec<String> {
  let f = File::open(file_name).unwrap();
  let reader = io::BufReader::new(f);
  let collect_lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
  println!("vec file lines: {:?}\n", collect_lines);
  collect_lines
}
