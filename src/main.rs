mod rdiff; // includes code from rdiff.rs 
use rdiff::run; // creates "named import" of the run function from the rdiff.rs module
use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("rdiff requires two file paths as arguments\n");
    return
  }

  // Place lines of file into a vec
  let f1 = file_to_vec(&args[1]);
  let f2 = file_to_vec(&args[2]);

  println!("--- {}", &args[1]);
  println!("+++ {}", &args[2]);

  // diff files
  run(f1, f2);
}

// convert file to vec of lines
fn file_to_vec(file_name: &str) -> Vec<String> {
  let f = File::open(file_name).unwrap();
  let reader = io::BufReader::new(f);
  let collect_lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
  collect_lines
}
