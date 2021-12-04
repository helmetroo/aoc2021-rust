use std::str::FromStr;

use regex::Regex;

use crate::puzzles::puzzle::Puzzle;

pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Command {
    direction: Direction,
    value: u8,
}

impl Command {
    pub fn new((direction, value): (Direction, u8)) -> Self {
        Self { direction, value }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(command_str: &str) -> Result<Self, Self::Err> {
        let command_regex = Regex::new(r"(?P<direction>forward|up|down)\s(?P<value>\d+)").unwrap();
        let captures = command_regex.captures(command_str).unwrap();
        let direction_str = captures.name("direction").unwrap().as_str();
        let direction = match direction_str {
            "forward" => Some(Direction::Forward),
            "down" => Some(Direction::Down),
            "up" => Some(Direction::Up),
            _ => None,
        };

        let value_str = captures.name("value").unwrap().as_str();
        let value = match value_str.parse::<u8>() {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        direction
            .zip(value)
            .map(Command::new)
            .ok_or(String::from(format!(
                "Couldn't parse command: `{}`",
                command_str
            )))
    }
}

fn compute_position(commands: &Vec<Command>) -> Position {
    let initial_position = Position { x: 0, y: 0 };
    commands
        .iter()
        .fold(initial_position, |acc, command| match command.direction {
            Direction::Forward => Position {
                x: acc.x + command.value as i32,
                ..acc
            },
            Direction::Down => Position {
                y: acc.y + command.value as i32,
                ..acc
            },
            Direction::Up => Position {
                y: acc.y - command.value as i32,
                ..acc
            },
        })
}

fn compute_position_with_aim(commands: &Vec<Command>) -> Position {
    let initial_position = Position { x: 0, y: 0 };
    let initial_aim = 0i32;
    let initial_conditions = (initial_position, initial_aim);

    commands
        .iter()
        .fold(initial_conditions, |conditions, command| {
            let (pos, aim) = conditions;
            match command.direction {
                Direction::Forward => (
                    Position {
                        x: pos.x + command.value as i32,
                        y: pos.y + (aim * command.value as i32),
                    },
                    aim,
                ),

                Direction::Down => (Position { ..pos }, aim + command.value as i32),

                Direction::Up => (Position { ..pos }, aim - command.value as i32),
            }
        })
        .0
}

pub struct P2;
impl Puzzle<Command> for P2 {
    fn number(&self) -> u8 {
        2
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<Command> {
        raw_data
            .iter()
            .map(|line| Command::from_str(line.as_str()).unwrap())
            .collect()
    }

    fn solve_part_one(&self, commands: &Vec<Command>) {
        let position = compute_position(commands);
        println!("{}", position.x * position.y)
    }

    fn solve_part_two(&self, commands: &Vec<Command>) {
        let position = compute_position_with_aim(commands);
        println!("{}", position.x * position.y)
    }
}
