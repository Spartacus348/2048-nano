use rand::{
    prelude::SliceRandom,
    rngs::ThreadRng,
    thread_rng};
use std::io::{stdin, Error, ErrorKind};
use std::ops::Range;

fn main() {
    println!("Hello, world!");
    let mut gen = thread_rng();
    let mut grid = init_grid(&mut gen);
    display_grid(&mut grid);
    while let Ok(x) = turn(&mut grid, &mut gen) {
        println!("Current score: {}", x)
    }
    println!("Good game!");
}

fn turn(grid: &mut Grid, gen: &mut ThreadRng) -> Result<u32, Error> {
    let direction = read_input()?;
    let merged: bool = step(grid, &direction);
    if get_empty_coords(grid).is_empty() {
        println!("Error: No more empty squares");
        return Err(Error::new(ErrorKind::Other, "No valid moves"));
    }
    if merged{ add_new_cells(grid, N_NEW_SQUARES, gen);}
    display_grid(grid);
    let base: u32 = 2;
    Ok(grid
        .iter()
        .map(|row: &[UVal; DIM]| {
            row.iter()
                .map(|&x: &UVal| if x == 0 { 0 } else { base.pow(x as u32) })
                .collect::<Vec<u32>>()
                .into_iter()
                .sum::<u32>()
        })
        .sum())
}

fn display_grid(grid: &mut Grid) {
    let base: u16 = 2;
    for row in grid.iter() {
        for val in row.iter() {
            if *val == 0 {
                print!("_ ");
            } else {
                print!("{} ", base.pow(*val as u32))
            }
        }
        println!();
    }
}

const N_NEW_SQUARES: usize = 1;
const DIM: usize = 4;
const AXES: Range<usize> = 0..DIM;

type UVal = u8;
type IVal = i8;

type Grid = [[UVal; DIM]; DIM];

fn get_val(grid: &Grid, coord: &Coord) -> UVal {
    grid[coord.x as usize][coord.y as usize]
}

fn put_val(grid: &mut Grid, coord: &Coord, val: UVal) {
    grid[coord.x as usize][coord.y as usize] = val;
}

enum MoveResult {
    Blocked,
    Merged,
    Through,
    NoEffect,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_input() -> Result<Direction, Error> {
    let mut default = Err(Error::new(ErrorKind::Other, "Default"));
    while let Err(_) = default {
        println!("Please input the move (w,a,s,d, x to exit):");
        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        default = match input.trim() {
            "w" => Ok(Direction::Left),
            "s" => Ok(Direction::Right),
            "a" => Ok(Direction::Up),
            "d" => Ok(Direction::Down),
            "x" => return Err(Error::new(ErrorKind::Other, "Exit game")),
            _ => {
                println!("Invalid input, try again");
                Err(Error::new(ErrorKind::InvalidInput, "Invalid move"))
            }
        }
    }
    default
}

struct Coord {
    x: IVal,
    y: IVal,
}

impl Coord {
    fn on_board(self: &Self) -> bool {
        self.x >= 0 && self.x < DIM as i8 && self.y >= 0 && self.y < DIM as i8
    }

    fn neighbor(self: &Self, direction: &Direction) -> Coord {
        match direction {
            Direction::Up => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn merge_ability(mover: UVal, target: UVal) -> MoveResult {
    if mover == 0 {
        MoveResult::NoEffect
    } else if target == 0 {
        MoveResult::Through
    } else if mover == target {
        MoveResult::Merged
    } else {
        MoveResult::Blocked
    }
}

fn merge_cells(mut grid: &mut Grid, move_idx: Coord, tgt_idx: Coord, direction: &Direction) -> bool{
    if !move_idx.on_board() || !tgt_idx.on_board() {
        return false;
    }

    let mover_cell = get_val(grid, &move_idx);
    let target_cell = get_val(grid, &tgt_idx);

    match merge_ability(mover_cell, target_cell) {
        MoveResult::Merged => {
            put_val(grid, &move_idx, 0);
            put_val(grid, &tgt_idx, target_cell + 1);
            true
        }
        MoveResult::Through => {
            put_val(grid, &tgt_idx, mover_cell);
            put_val(grid, &move_idx, 0);
            let new_tgt_idx = tgt_idx.neighbor(direction);
            merge_cells(&mut grid, tgt_idx, new_tgt_idx, direction);
            true
        }
        _ => {
            false
        }
    }
}

fn step(mut grid: &mut Grid, direction: &Direction) -> bool
{
    let mut merged = false;
    match direction {
        Direction::Up => {
            for y in AXES {
                for x in AXES {
                    let move_idx = Coord {
                        x: x as IVal,
                        y: y as IVal,
                    };
                    let tgt_idx = move_idx.neighbor(direction);
                    merged |= merge_cells(&mut grid, move_idx, tgt_idx, direction);
                }
            }
        }
        Direction::Down => {
            for y in AXES.rev() {
                for x in AXES {
                    let move_idx = Coord {
                        x: x as IVal,
                        y: y as IVal,
                    };
                    let tgt_idx = move_idx.neighbor(direction);
                    merged |= merge_cells(&mut grid, move_idx, tgt_idx, direction);

                }
            }
        }
        Direction::Left => {
            for x in AXES {
                for y in AXES {
                    let move_idx = Coord {
                        x: x as IVal,
                        y: y as IVal,
                    };
                    let tgt_idx = move_idx.neighbor(direction);
                    merged |= merge_cells(&mut grid, move_idx, tgt_idx, direction);
                }
            }
        }
        Direction::Right => {
            for x in AXES.rev() {
                for y in AXES {
                    let move_idx = Coord {
                        x: x as IVal,
                        y: y as IVal,
                    };
                    let tgt_idx = move_idx.neighbor(direction);
                    merged |= merge_cells(&mut grid, move_idx, tgt_idx, direction);
                }
            }
        }
    }
    merged
}

fn get_empty_coords(grid: &Grid) -> Vec<Coord> {
    let mut coords = vec![];
    for y in AXES {
        for x in AXES {
            if get_val(
                grid,
                &Coord {
                    x: x as i8,
                    y: y as i8,
                },
            ) == 0
            {
                coords.push(Coord {
                    x: x as i8,
                    y: y as i8,
                });
            }
        }
    }
    coords
}

fn add_new_cells(grid: &mut Grid, num: usize, gen: &mut ThreadRng) {
    let options = get_empty_coords(grid);
    for new_idx in options.choose_multiple(gen, num) {
        put_val(grid, new_idx, 1);
    }
}

fn init_grid(gen: &mut ThreadRng) -> Grid {
    let mut grid = [[0; DIM]; DIM];
    add_new_cells(&mut grid, 2, gen);
    grid
}