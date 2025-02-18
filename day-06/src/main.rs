use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Copy, Clone)]
enum MazeCell {
    Empty { visited: bool },
    Wall,
    Guard { direction: Direction },
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Hash)]
enum MazeInfo {
    IsDone,
    NotDone {
        guard_row: usize,
        guard_col: usize,
        guard_direction: Direction,
    },
}

impl Direction {
    fn turned(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn step(maze: &mut Vec<Vec<MazeCell>>) -> MazeInfo {
    let max_row = maze.len() - 1;
    let max_col = maze[0].len() - 1;
    let mut guard_row: Option<usize> = None;
    let mut guard_col: Option<usize> = None;
    let mut guard_direction: Option<Direction> = None;
    for i in 0..max_row + 1 {
        for j in 0..max_col + 1 {
            match maze[i][j] {
                MazeCell::Guard { direction } => {
                    (guard_row, guard_col, guard_direction) = (Some(i), Some(j), Some(direction));
                }
                _ => {}
            }
        }
    }
    let guard_row = guard_row.expect("No guard!") as i32;
    let guard_col = guard_col.expect("No guard!") as i32;
    let guard_direction = guard_direction.expect("No guard!");

    let (d_row, d_col);
    match guard_direction {
        Direction::Up => {
            (d_row, d_col) = (-1, 0);
        }
        Direction::Right => {
            (d_row, d_col) = (0, 1);
        }
        Direction::Down => {
            (d_row, d_col) = (1, 0);
        }
        Direction::Left => {
            (d_row, d_col) = (0, -1);
        }
    }

    let (target_row, target_col) = (guard_row as i32 + d_row, guard_col as i32 + d_col);

    if !(0 <= target_row
        && target_row <= max_row as i32
        && 0 <= target_col
        && target_col <= max_col as i32)
    {
        maze[guard_row as usize][guard_col as usize] = MazeCell::Empty { visited: true };
        return MazeInfo::IsDone;
    }

    let guard_row = guard_row as usize;
    let guard_col = guard_col as usize;
    let target_row = target_row as usize;
    let target_col = target_col as usize;

    match maze[target_row][target_col] {
        MazeCell::Empty { .. } => {
            maze[guard_row][guard_col] = MazeCell::Empty { visited: true };
            maze[target_row][target_col] = MazeCell::Guard {
                direction: guard_direction,
            };
            return MazeInfo::NotDone {
                guard_row: target_row,
                guard_col: target_col,
                guard_direction: guard_direction,
            };
        }
        MazeCell::Wall => {
            maze[guard_row][guard_col] = MazeCell::Guard {
                direction: guard_direction.turned(),
            };
            return MazeInfo::NotDone {
                guard_row: guard_row,
                guard_col: guard_col,
                guard_direction: guard_direction.turned(),
            };
        }
        MazeCell::Guard { .. } => {
            panic!("how did we get 2 guards?")
        }
    }
}

fn part_1(maze: &Vec<Vec<MazeCell>>) {
    let mut maze = maze.clone();
    loop {
        match step(&mut maze) {
            MazeInfo::IsDone => {
                break;
            }
            _ => {}
        }
    }
    let mut visited_count = 0;
    maze.iter().flatten().for_each(|c| match c {
        MazeCell::Empty { visited: true } => {
            visited_count += 1;
        }
        _ => {}
    });
    println!("{visited_count}");
}

fn part_2(maze: &Vec<Vec<MazeCell>>) {
    let mut num_working_positions = 0;
    let mut tracker = HashSet::new();
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            tracker.clear();
            match maze[i][j] {
                MazeCell::Empty { .. } => {
                    let mut maze = maze.clone();
                    maze[i][j] = MazeCell::Wall;
                    loop {
                        match step(&mut maze) {
                            MazeInfo::IsDone => {
                                break;
                            }
                            MazeInfo::NotDone {
                                guard_row,
                                guard_col,
                                guard_direction,
                            } => {
                                let info = MazeInfo::NotDone {
                                    guard_row: guard_row,
                                    guard_col: guard_col,
                                    guard_direction: guard_direction,
                                };
                                if tracker.contains(&info) {
                                    num_working_positions += 1;
                                    break;
                                }
                                tracker.insert(info);
                            }
                        };
                    }
                    maze[i][j] = MazeCell::Empty { visited: false };
                }
                _ => {}
            }
        }
    }
    println!("{num_working_positions}");
}

fn main() {
    let input = std::fs::read_to_string("./input.txt")
        .expect("input file missing or in the wrong location");

    let maze: Vec<Vec<MazeCell>> = input
        .lines()
        .map(|line| {
            line.graphemes(true)
                .map(|g| match g {
                    "." => MazeCell::Empty { visited: false },
                    "#" => MazeCell::Wall,
                    "^" => MazeCell::Guard {
                        direction: Direction::Up,
                    },
                    _ => panic!("malformed maze input"),
                })
                .collect()
        })
        .collect();

    part_1(&maze);
    part_2(&maze);
}
