#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs;

fn validate_int_range(value: &str, min: isize, max: isize) -> bool {
  match value.parse::<isize>() {
    Ok(v) => {
      if v >= min && v <= max {
        true
      } else {
        false
      }
    }
    Err(_) => false,
  }
}

fn validate_hgt(value: &str) -> bool {
  if value.ends_with("cm") {
    match value.get(0..value.len() - 2).unwrap().parse::<isize>() {
      Ok(v) => {
        if v >= 150 && v <= 193 {
          true
        } else {
          false
        }
      }
      Err(_) => false,
    }
  } else if value.ends_with("in") {
    match value.get(0..value.len() - 2).unwrap().parse::<isize>() {
      Ok(v) => {
        if v >= 59 && v <= 76 {
          true
        } else {
          false
        }
      }
      Err(_) => false,
    }
  } else {
    false
  }
}

fn validate_hcl(value: &str) -> bool {
  lazy_static! {
    static ref HCL_RE: Regex = Regex::new(r"#[\da-f]{6}").unwrap();
  }
  HCL_RE.is_match(value)
}

fn validate_ecl(value: &str) -> bool {
  match value {
    "amb" => true,
    "blu" => true,
    "brn" => true,
    "gry" => true,
    "grn" => true,
    "hzl" => true,
    "oth" => true,
    _ => false,
  }
}

fn validate_pid(value: &str) -> bool {
  lazy_static! {
    static ref PID_RE: Regex = Regex::new(r"\d{9}").unwrap();
  }
  PID_RE.is_match(value)
}

fn read_field(text: &str) -> (String, String) {
  let mut field = String::new();
  let mut value = String::new();

  let mut chars = text.chars();

  while field.len() < 3 {
    field.push(chars.next().unwrap());
  }

  chars.next();

  for c in chars {
    value.push(c);
  }

  (field, value)
}

fn main() {
  let input = fs::read_to_string("input").unwrap();
  let mut valid_passports_count = 0;
  let mut valid_fields_count = 0;

  for line in input.lines() {
    if line == "" {
      // New passport, reset variables
      if valid_fields_count == 7 {
        valid_passports_count += 1;
      }
      valid_fields_count = 0;
    } else {
      for pair in line.split(' ') {
        let (key, value) = read_field(pair);
        if match key.as_str() {
          "byr" => validate_int_range(&value, 1920, 2002),
          "iyr" => validate_int_range(&value, 2010, 2020),
          "eyr" => validate_int_range(&value, 2020, 2030),
          "hgt" => validate_hgt(&value),
          "hcl" => validate_hcl(&value),
          "ecl" => validate_ecl(&value),
          "pid" => validate_pid(&value),
          _ => false,
        } {
          valid_fields_count += 1;
        }
      }
    }
  }
  println!("Valid count: {}", valid_passports_count);
}
