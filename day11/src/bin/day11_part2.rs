use std::fmt::{self, Display, Formatter, Write};
use std::fs;
use std::io::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Seat {
    Free,
    Taken,
    Floor,
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Seat::Free => "L",
                Seat::Taken => "#",
                Seat::Floor => ".",
            }
        )
    }
}

#[derive(Debug)]
struct GridCell {
    value: Seat,
    position: (usize, usize),
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<GridCell>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell.value)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Grid {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    /// Counts the number of free adjacent cells, given a cell
    pub fn occupied_adjacent_cells_count(&self, cell: &GridCell) -> usize {
        let mut occupied_count = 0;

        let search_rules: Vec<(isize, isize)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for rule in search_rules {
            let mut next_pos = (cell.position.0 as isize, cell.position.1 as isize);
            loop {
                next_pos = (next_pos.0 + rule.0, next_pos.1 + rule.1);

                if next_pos.0 < 0
                    || next_pos.0 > self.cells.len() as isize - 1
                    || next_pos.1 < 0
                    || next_pos.1 > self.cells[0].len() as isize - 1
                {
                    break;
                }

                let current_cell = &self.cells[next_pos.0 as usize][next_pos.1 as usize];

                match current_cell.value {
                    Seat::Free => {
                        break;
                    }
                    Seat::Taken => {
                        occupied_count += 1;
                        break;
                    }
                    Seat::Floor => (),
                }
            }
        }

        occupied_count
    }

    pub fn total_occupied_count(&self) -> usize {
        let mut count = 0;

        for row in self.cells.iter() {
            for cell in row.iter() {
                if cell.value == Seat::Taken {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day11/input")?;
    let mut debug = String::new();

    // setup seats in a grid
    let mut grid = Grid::new();
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, chr) in line.chars().enumerate() {
            match chr {
                'L' => row.push(GridCell {
                    value: Seat::Free,
                    position: (i, j),
                }),
                '.' => row.push(GridCell {
                    value: Seat::Floor,
                    position: (i, j),
                }),
                _ => panic!("Invalid input char"),
            }
        }
        grid.cells.push(row);
    }

    // loop until there are no changes
    loop {
        let mut did_change = false;

        let mut next_grid = Grid::new();

        for (i, row) in grid.cells.iter().enumerate() {
            next_grid.cells.push(Vec::new());
            for (j, cell) in row.iter().enumerate() {
                if cell.value == Seat::Floor {
                    next_grid.cells[i].push(GridCell {
                        position: (i, j),
                        value: Seat::Floor,
                    });
                    continue;
                }
                let occupied_adjacent = grid.occupied_adjacent_cells_count(cell);
                if cell.value == Seat::Free {
                    if occupied_adjacent == 0 {
                        next_grid.cells[i].push(GridCell {
                            position: (i, j),
                            value: Seat::Taken,
                        });
                        did_change = true;
                    } else {
                        next_grid.cells[i].push(GridCell {
                            position: (i, j),
                            value: Seat::Free,
                        });
                    }
                } else {
                    if occupied_adjacent >= 5 {
                        next_grid.cells[i].push(GridCell {
                            position: (i, j),
                            value: Seat::Free,
                        });
                        did_change = true;
                    } else {
                        next_grid.cells[i].push(GridCell {
                            position: (i, j),
                            value: Seat::Taken,
                        });
                    }
                }
            }
        }

        writeln!(debug, "--- Next round ---\n{}", next_grid).unwrap();
        grid = next_grid;
        if !did_change {
            break;
        }
    }

    println!("Final taken seats count: {}", grid.total_occupied_count());

    fs::write("day11/debug", debug)?;

    Ok(())
}
