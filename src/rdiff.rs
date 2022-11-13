// Myers diff algorithm implementation

#[derive(Clone)]
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

pub fn run(first_text: Vec<String>, second_text: Vec<String>) {
  let mut edit_trace = lcs(&first_text, &second_text);
  ses(&mut edit_trace, &first_text, &second_text);
}

// lcs produces the shortest edit sequence graph, given two files 
fn lcs(a: &Vec<String>, b: &Vec<String>) -> Vec<CircularArray> {
  // *** SETUP ***
  let n = a.len();
  let m = b.len();
  let max = n + m;

  // k: (x - y)
  // d: depth (iteration)
  // pos: track x value for each k (circular array)

  // Create a new slice to track max x values

  let mut pos = CircularArray::new(2 * max + 1);
  pos.set(1, 0);

  // *** iterate over depth and k value ***

  // Track values for all iterations => the diff graph
  let mut trace: Vec<CircularArray> = vec![];

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
      while x < n as i32 && y < m as i32 && a[x as usize] == b[y as usize] {
        x = x + 1;
        y = y + 1;
      }

      pos.set(k, x);

      if x >= n as i32 && y >= m as i32 {
        return trace;
      }

    }
    trace.push(pos.clone());
  }
  // returns -1 if there is an error
  //-1 
  vec![]
}

// ses backtracks through a given diff graph to display the edit sequence
fn ses(trace: &mut Vec<CircularArray>, a: &Vec<String>, b: &Vec<String>) {
  let mut diff: Vec<String> = vec![];

  let mut x = a.len() as i32;
  let mut y = b.len() as i32;

  let mut d = trace.len() as i32;
  let mut d_pos = trace.pop(); // pop returns an Option<CircularArray> => Needs to be unwrapped to get underlying value

  while d_pos.is_some() {
    let pos = d_pos.unwrap();
    let k = x - y;

    let prev_k: i32;
    if k == (-1 * d) || (k != d && pos.get(k - 1) < pos.get(k + 1)) {
      prev_k = k + 1;
    } else {
      prev_k = k - 1;
    }

    let prev_x = pos.get(prev_k);
    let prev_y = prev_x - prev_k;

    // Trace diagonals in path if they exist
    while x > prev_x && y > prev_y {
      diff.push(format!("  {}", &a[(x - 1) as usize]));
      x = x - 1; 
      y = y - 1;
    }

    if d > 0 {
      if x == prev_x {
        diff.push(format!("+ {}", &b[(y - 1) as usize]));
      } else if y == prev_y {
        diff.push(format!("- {}", &a[(x - 1) as usize]));
      } else {
        diff.push(format!("  {}", &a[(x - 1) as usize]));
      }
    }

    x = prev_x;
    y = prev_y;

    // Checkout x values for the previous depth iteration
    d = trace.len() as i32;
    d_pos = trace.pop();
  }

  // display the diff
  let mut next_line = diff.pop();
  while next_line.is_some() {
    let line = next_line.unwrap();
    println!("{}", line);
    next_line = diff.pop();
  }

}
