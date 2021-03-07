use std::fs;
use std::io::Error;

fn max(v1: isize, v2: isize) -> isize {
  if v1 > v2 {
    v1
  } else {
    v2
  }
}

fn insort(vec: &mut Vec<usize>, val: usize) {
  let mut index = max(vec.len() as isize, 0) as usize;
  for (i, v) in vec.iter().enumerate() {
    if val < *v {
      index = i;
      break;
    }
  }

  vec.insert(index, val);
}

fn missing(vec: Vec<usize>) -> usize {
  let mut missing_val = 0;

  let mut vec_iter = vec.iter();
  let mut last_val = vec_iter.next().unwrap();

  for v in vec_iter {
    if *last_val != v - 1 {
      missing_val = v - 1;
      break;
    }
    last_val = v;
  }

  missing_val
}

fn main() -> Result<(), Error> {
  let input = fs::read_to_string("day5/input")?;
  let mut debug = String::new();
  let mut seats = Vec::<usize>::new();

  for line in input.lines() {
    let (mut start, mut end, mut mid) = (0, 127, 63);
    debug.push_str(line);
    let mut instructions = line.chars();

    for _ in 0..7 {
      let next_instruction = instructions.next().expect("Early row end");
      match next_instruction {
        'F' => {
          end = mid;
          mid = (start + end) / 2;
        }
        'B' => {
          start = mid + 1;
          mid = (start + end) / 2;
        }
        _ => panic!("Unexpected row value {}", next_instruction),
      }
    }

    if start != end {
      panic!("Unexpected row {}-{}", start, end);
    }

    let row = start;

    let (mut start, mut end, mut mid) = (0, 7, 3);
    for _ in 0..3 {
      let next_instruction = instructions.next().expect("Early col end");
      match next_instruction {
        'L' => {
          end = mid;
          mid = (start + end) / 2;
        }
        'R' => {
          start = mid + 1;
          mid = (start + end) / 2;
        }
        _ => panic!("Unexpected col value {}", next_instruction),
      }
    }

    if start != end {
      panic!("Unexpected col {}-{}", start, end);
    }
    let id = row * 8 + start;
    insort(&mut seats, id);

    debug.push_str(&format!(" - row {} col {}\n", row, start));
  }

  // Check missing seat
  println!("Missing seat is {}", missing(seats));

  fs::write("debug", debug)?;

  Ok(())
}
