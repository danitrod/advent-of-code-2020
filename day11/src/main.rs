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
        let hits_x_start = cell.position.0 == 0;
        let hits_x_end = cell.position.0 == self.cells.len() - 1;
        let hits_y_start = cell.position.1 == 0;
        let hits_y_end = cell.position.1 == self.cells[0].len() - 1;

        let mut occupied_count = 0;

        if !hits_x_start {
            if !hits_y_start {
                if self.cells[cell.position.0 - 1][cell.position.1 - 1].value == Seat::Taken {
                    occupied_count += 1;
                }
            }
            if self.cells[cell.position.0 - 1][cell.position.1].value == Seat::Taken {
                occupied_count += 1;
            }
            if !hits_y_end {
                if self.cells[cell.position.0 - 1][cell.position.1 + 1].value == Seat::Taken {
                    occupied_count += 1;
                }
            }
        }

        if !hits_x_end {
            if !hits_y_start {
                if self.cells[cell.position.0 + 1][cell.position.1 - 1].value == Seat::Taken {
                    occupied_count += 1;
                }
            }
            if self.cells[cell.position.0 + 1][cell.position.1].value == Seat::Taken {
                occupied_count += 1;
            }
            if !hits_y_end {
                if self.cells[cell.position.0 + 1][cell.position.1 + 1].value == Seat::Taken {
                    occupied_count += 1;
                }
            }
        }

        if !hits_y_end {
            if self.cells[cell.position.0][cell.position.1 + 1].value == Seat::Taken {
                occupied_count += 1;
            }
        }
        if !hits_y_start {
            if self.cells[cell.position.0][cell.position.1 - 1].value == Seat::Taken {
                occupied_count += 1;
            }
        }

        occupied_count
    }

    pub fn occupied_count(&self) -> usize {
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
                } else if cell.value == Seat::Taken {
                    if occupied_adjacent >= 4 {
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
                } else {
                    next_grid.cells[i].push(GridCell {
                        position: (i, j),
                        value: Seat::Floor,
                    });
                }
            }
        }

        writeln!(debug, "--- Next round ---\n{}", next_grid).unwrap();
        grid = next_grid;
        if !did_change {
            break;
        }
    }

    println!("Final taken seats count: {}", grid.occupied_count());

    fs::write("day11/debug", debug)?;

    Ok(())
}
