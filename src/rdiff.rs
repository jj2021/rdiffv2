// Myers diff algorithm implementation

/*
// This circular array implementation works!!! 
struct CircArray {
  arr: Vec<i32>,
}

impl CircArray {
  fn new(size: usize) -> Self {
    Self {
      arr: vec![-1; size],
    }
  }

  fn get(&self, index: i32) -> i32 {
    if index < 0 {
      let real_index = self.arr.len() as i32 + index;
      return self.arr[real_index as usize];
    }
    self.arr[index as usize]
  }

  fn set(&mut self, index: i32, value: i32) {
    if index < 0 {
      let real_index = self.arr.len() as i32 + index;
      self.arr[real_index as usize] = value;
    } else {
      self.arr[index as usize] = value;
    }
  }
}
*/

// get value by index of slice, including negative indicies
fn get_value(record: &[i32], index: i32) -> i32 {
    if index < 0 {
      let real_index = record.len() as i32 + index;
      return record[real_index as usize];
    }
    record[index as usize]
}

// set value by index of slice, including negative indicies
fn set_value(record: &mut [i32], index: i32, value: i32) {
    if index < 0 {
      let real_index = record.len() as i32 + index;
      record[real_index as usize] = value;
    } else {
      record[index as usize] = value;
    }
}

// TODO: Implement Longest Common Subsequence method
fn lcs(a: &str, b: &str) -> i32  {
  // *** SETUP ***
  let n = a.chars().count(); // len() returns num bytes, not num characters
  let m = b.chars().count();
  println!("a is {} char long and b is {} char long", n, m);
  let max = n + m;
  println!("the max edits is {}", max);

  // k: (x - y)
  // d: depth (iteration)
  // pos: track x value for each k (circular array)

  // Create a new slice to track max x values

  let mut v: Vec<i32> = vec![-1; 2 * max + 1];
  let pos = v.as_mut_slice();
  println!("pos length {}", pos.len());
  pos[1] = 0;

  // *** iterate over depth and k value ***

  // max is of type usize, so we have to cast to i32 to allow k to be negative
  // maybe try using isize type instead of i32???
  for d in 0..=max as i32 {
    for k in (-1 * d..=d).step_by(2) {
      let mut x: i32;
      if k == (-1 * d) || (k != d && get_value(pos, k - 1) < get_value(pos, k + 1)) {
        x = get_value(pos, k + 1);
      } else {
        x = get_value(pos, k - 1) + 1;
      }

      let mut y = x - k;

      // find diagonal moves
      while x < n as i32 && y < m as i32 && a.chars().nth(x as usize) == b.chars().nth(y as usize) {
        x = x + 1;
        y = y + 1;
      }

      set_value(pos, k, x);

      if x >= n as i32 && y >= m as i32 {
        return d;
      }

    }
  }
  // returns -1 if there is an error
  -1 
}

// TODO: Implement backtracking (shortest edit sequence)
fn ses() -> i32 {
  0
}

// TODO: Utilize diff logic in a CLI applicaation
pub fn run() {
  let _ = ses();

  //test_circular_array();

  //let _ = lcs("hello", "there");
  println!("shortest edit distance: {}", lcs("hello", "there"));

  //test_circ_array();
}

/*
fn test_circular_array() {
  let mut v = vec![-1;5];
  let p = v.as_mut_slice();
  println!("{:?}", p);

  set_value(p, -3, 5);
  println!("{:?}", p);
  
  let value = get_value(p, -3);
  println!("value at index -3 is {}", value);
}
*/

/*
fn test_circ_array() {
  let mut circ = CircArray::new(5);
  println!("circ: {:?}", circ.arr);
  circ.arr[0] = 5;
  println!("circ: {:?}", circ.arr);

  let mut circ2 = CircArray::new(5);
  println!("circ2: {:?}", circ2.arr);
  circ2.set(-2, 5);
  println!("circ2: {:?}", circ2.arr);
  println!("value at index -2 in circ2 {:?}", circ2.get(-2));
}
*/