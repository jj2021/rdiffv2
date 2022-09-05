// Myers diff algorithm implementation

struct CircularArray {
  arr: Vec<i32>,
}

impl CircularArray {
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

  let mut pos = CircularArray::new(2 * max + 1);

  // *** iterate over depth and k value ***

  // max is of type usize, so we have to cast to i32 to allow k to be negative
  // maybe try using isize type instead of i32???
  for d in 0..=max as i32 {
    for k in (-1 * d..=d).step_by(2) {
      let mut x: i32;
      if k == (-1 * d) || (k != d && pos.get(k - 1) < pos.get(k + 1)) {
        x = pos.get(k + 1);
      } else {
        x = pos.get(k - 1) + 1;
      }

      let mut y = x - k;

      // find diagonal moves
      while x < n as i32 && y < m as i32 && a.chars().nth(x as usize) == b.chars().nth(y as usize) {
        x = x + 1;
        y = y + 1;
      }

      pos.set(k, x);

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

  println!("shortest edit distance: {}", lcs("hello", "there"));
}