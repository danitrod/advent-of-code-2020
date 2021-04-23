use std::fmt::Write;
use std::fs;
use std::io::Error;

enum Direction {
    East,
    South,
    West,
    North,
}

enum TurnDirection {
    Left,
    Right,
}

impl Direction {
    pub fn add_value(&self, val: isize, counter: (isize, isize)) -> (isize, isize) {
        match &self {
            Direction::East => (counter.0 + val, counter.1),
            Direction::South => (counter.0, counter.1 - val),
            Direction::West => (counter.0 - val, counter.1),
            Direction::North => (counter.0, counter.1 + val),
        }
    }

    pub fn right(&self) -> Direction {
        match &self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }

    pub fn left(&self) -> Direction {
        match &self {
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
        }
    }
}

struct Boat {
    facing_dir: Direction,
    distance: (isize, isize),
}

impl Boat {
    pub fn new() -> Self {
        Boat {
            facing_dir: Direction::East,
            distance: (0, 0),
        }
    }

    pub fn change_direction(&mut self, degrees: usize, dir: TurnDirection) {
        let turns = degrees / 90;
        for _ in 0..turns {
            match dir {
                TurnDirection::Left => {
                    self.facing_dir = self.facing_dir.left();
                }
                TurnDirection::Right => {
                    self.facing_dir = self.facing_dir.right();
                }
            }
        }
    }

    pub fn forward(&mut self, val: usize) {
        self.distance = self.facing_dir.add_value(val as isize, self.distance);
    }

    pub fn move_to(&mut self, dir: Direction, val: usize) {
        self.distance = dir.add_value(val as isize, self.distance);
    }
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day12/input")?;
    let mut debug = String::new();
    let mut boat = Boat::new();

    for line in input.lines() {
        let mut chars = line.chars();
        let command = chars.next().expect("Invalid line command");
        let value: usize = chars
            .collect::<String>()
            .parse()
            .expect("Invalid line value");

        match command {
            'N' => boat.move_to(Direction::North, value),
            'S' => boat.move_to(Direction::South, value),
            'E' => boat.move_to(Direction::East, value),
            'W' => boat.move_to(Direction::West, value),
            'L' => boat.change_direction(value, TurnDirection::Left),
            'R' => boat.change_direction(value, TurnDirection::Right),
            'F' => boat.forward(value),
            _ => panic!("Unexpected command"),
        }

        writeln!(debug, "Boat now on {:?}", boat.distance).unwrap();
    }

    println!("Final distance: {:?}", boat.distance);
    println!(
        "Manhattan distance: {}",
        boat.distance.0.abs() + boat.distance.1.abs()
    );

    fs::write("day12/debug", debug)?;

    Ok(())
}
