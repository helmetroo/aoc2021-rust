use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::puzzles::puzzle::Puzzle;

type MinMax = (i32, i32);
pub struct Bounds {
    x: MinMax,
    y: MinMax,
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    pub fn new() -> Self {
        Position { x: 0, y: 0 }
    }
}

struct Trajectory {
    max_y: i32,
    hit_bounds: bool,
}

lazy_static! {
    static ref NEGATIVE_INFINITY: i32 = i32::MIN;
    static ref BOUNDS_REGEX: Regex = Regex::new(
        r"target area: x=(?P<x_min>-?\d+)..(?P<x_max>-?\d+), y=(?P<y_min>-?\d+)..(?P<y_max>-?\d+)"
    )
    .unwrap();
}

type BoundsParseErr = &'static str;
impl FromStr for Bounds {
    type Err = BoundsParseErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        BOUNDS_REGEX
            .captures(line)
            .as_ref()
            .map(extract_bounds)
            .ok_or("Failed to parse bounds")
    }
}

fn parse_match_as_int(m: regex::Match) -> i32 {
    m.as_str().parse().expect("Failed to parse integer")
}

fn extract_bounds(captures: &regex::Captures) -> Bounds {
    let x_min = captures.name("x_min").map(parse_match_as_int);
    let x_max = captures.name("x_max").map(parse_match_as_int);
    let xs = x_min
        .zip(x_max)
        .expect("Failed to parse both x_min and x_max");

    let y_min = captures.name("y_min").map(parse_match_as_int);
    let y_max = captures.name("y_max").map(parse_match_as_int);
    let ys = y_min
        .zip(y_max)
        .expect("Failed to parse both y_min and y_max");

    Bounds { x: xs, y: ys }
}

fn find_max_y_over_trajectories(bounds: &Bounds) -> i32 {
    /*
      Yes, it's a brute force search, but I wanted to prune the search bounds to make it run a bit faster by making some educated stabs in the dark.
      i.e. Smallest x velocity is sqrt(2*x_min), as this follows from parabolic projectile motion (x' = x_vel - t) => (x = x_vel*t - t^2 / 2)
    */

    let smallest_x_vel = (2f32 * bounds.x.0 as f32).sqrt().floor() as i32;
    let x_velocities = smallest_x_vel..bounds.x.1;
    let y_velocities = bounds.y.1..(-bounds.y.0);
    let mut max_y = *NEGATIVE_INFINITY;
    for x_vel in x_velocities {
        for y_vel in y_velocities.clone() {
            let Trajectory {
                max_y: cur_max_y,
                hit_bounds,
            } = build_trajectory(bounds, x_vel, y_vel);
            if hit_bounds {
                if cur_max_y >= max_y {
                    max_y = cur_max_y
                }
            }
        }
    }

    max_y
}

fn build_trajectory(bounds: &Bounds, x_vel: i32, y_vel: i32) -> Trajectory {
    let mut keep_going = true;
    let mut hit_bounds = false;
    let mut path = Vec::new();
    let mut position = Position::new();
    let mut cur_x_vel = x_vel;
    let mut cur_y_vel = y_vel;
    while keep_going {
        path.push(position.clone());
        position.x += cur_x_vel;
        position.y += cur_y_vel;
        if cur_x_vel > 0 {
            cur_x_vel -= 1;
        }
        cur_y_vel -= 1;

        let not_missed_bounds = (position.x < bounds.x.1) && (position.y > bounds.y.0);
        hit_bounds = (bounds.x.0 <= position.x && position.x <= bounds.x.1)
            && (bounds.y.0 <= position.y && position.y <= bounds.y.1);

        keep_going = not_missed_bounds && !hit_bounds;
    }

    let max_y = path.iter().max_by_key(|pos| pos.y).unwrap().y;
    Trajectory { max_y, hit_bounds }
}

pub struct P17;
impl Puzzle<Bounds> for P17 {
    fn number(&self) -> u8 {
        17
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Bounds {
        Bounds::from_str(&raw_data[0]).unwrap()
    }

    fn solve_part_one(&self, bounds: &Bounds) {
        let max_y = find_max_y_over_trajectories(bounds);
        println!("{}", max_y);
    }

    fn solve_part_two(&self, _bounds: &Bounds) {}
}
