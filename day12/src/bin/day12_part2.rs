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
}

struct Boat {
    distance: (isize, isize),
    waypoint: (isize, isize),
}

impl Boat {
    pub fn new() -> Self {
        Boat {
            distance: (0, 0),
            waypoint: (10, 1),
        }
    }

    pub fn forward(&mut self, value: isize) {
        self.distance.0 += value * self.waypoint.0;
        self.distance.1 += value * self.waypoint.1;
    }

    pub fn move_waypoint(&mut self, dir: Direction, val: isize) {
        self.waypoint = dir.add_value(val, self.waypoint);
    }

    pub fn rotate_waypoint(&mut self, dir: TurnDirection, degrees: isize) {
        let times = degrees / 90;

        let rotation: Box<dyn Fn((isize, isize)) -> (isize, isize)> = match dir {
            TurnDirection::Left => Box::new(|w: (isize, isize)| (-w.1, w.0)),
            TurnDirection::Right => Box::new(|w: (isize, isize)| (w.1, -w.0)),
        };

        for _ in 0..times {
            self.waypoint = rotation(self.waypoint);
        }
    }
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day12/input")?;
    let mut debug = String::new();
    let mut boat = Boat::new();

    for line in input.lines() {
        let mut chars = line.chars();
        let command = chars.next().expect("Invalid line command");
        let value: isize = chars
            .collect::<String>()
            .parse()
            .expect("Invalid line value");

        match command {
            'N' => boat.move_waypoint(Direction::North, value),
            'S' => boat.move_waypoint(Direction::South, value),
            'E' => boat.move_waypoint(Direction::East, value),
            'W' => boat.move_waypoint(Direction::West, value),
            'L' => boat.rotate_waypoint(TurnDirection::Left, value),
            'R' => boat.rotate_waypoint(TurnDirection::Right, value),
            'F' => boat.forward(value),
            _ => panic!("Unexpected command"),
        }

        writeln!(
            debug,
            r"
Instruction was {}{}
Boat now on {:?}
Waypoint now is {:?}
",
            command, value, boat.distance, boat.waypoint
        )
        .unwrap();
    }

    println!("Final distance: {:?}", boat.distance);
    println!(
        "Manhattan distance: {}",
        boat.distance.0.abs() + boat.distance.1.abs()
    );

    fs::write("day12/debug", debug)?;

    Ok(())
}
