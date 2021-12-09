use std::iter;
use std::str::FromStr;

use regex::Regex;

use crate::puzzles::puzzle::Puzzle;

pub struct Position {
    x: i16,
    y: i16,
}

pub enum Coordinate {
    X,
    Y,
}

pub struct LineSegment {
    start: Position,
    end: Position,
}

type LineSegmentParseErr = &'static str;

impl LineSegment {
    pub fn is_vertical(&self) -> bool {
        self.end.x == self.start.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.end.y == self.start.y
    }

    pub fn is_vertical_or_horizontal(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    pub fn build_walk_list(&self) -> Vec<Position> {
        let x = Coordinate::X;
        let y = Coordinate::Y;
        let min_x = LineSegment::min_coord_for_line(&self, &x);
        let min_y = LineSegment::min_coord_for_line(&self, &y);
        let max_x = LineSegment::max_coord_for_line(&self, &x);
        let max_y = LineSegment::max_coord_for_line(&self, &y);

        let along_x = min_x..max_x + 1;
        let along_y = min_y..max_y + 1;
        let is_vertical = self.is_vertical();
        let (shorter, longer) = if is_vertical {
            (along_x, along_y)
        } else {
            (along_y, along_x)
        };

        longer
            .into_iter()
            .zip(iter::repeat(shorter.start))
            .map(|(x, y)| {
                if is_vertical {
                    Position { x: y, y: x }
                } else {
                    Position { x, y }
                }
            })
            .collect::<Vec<Position>>()
    }

    pub fn max_coord_for_lines(lines: &Vec<&LineSegment>, coord: &Coordinate) -> i16 {
        lines.into_iter().fold(i16::MIN, |max, line| {
            let cur_max_coord = max;
            let max_coord = LineSegment::max_coord_for_line(line, &coord);
            if max_coord >= cur_max_coord {
                max_coord
            } else {
                cur_max_coord
            }
        })
    }

    fn min_coord_for_line(line: &LineSegment, coord: &Coordinate) -> i16 {
        let is_x_coord = matches!(coord, Coordinate::X);
        let start = if is_x_coord {
            line.start.x
        } else {
            line.start.y
        };
        let end = if is_x_coord { line.end.x } else { line.end.y };
        if start <= end {
            start
        } else {
            end
        }
    }

    fn max_coord_for_line(line: &LineSegment, coord: &Coordinate) -> i16 {
        let is_x_coord = matches!(coord, Coordinate::X);
        let start = if is_x_coord {
            line.start.x
        } else {
            line.start.y
        };
        let end = if is_x_coord { line.end.x } else { line.end.y };
        if start >= end {
            start
        } else {
            end
        }
    }
}

impl FromStr for LineSegment {
    type Err = LineSegmentParseErr;
    fn from_str(line_str: &str) -> Result<Self, Self::Err> {
        let line_regex = Regex::new(r"(?P<x0>\d+),(?P<y0>\d+) -> (?P<x1>\d+),(?P<y1>\d+)").unwrap();
        let captures = line_regex.captures(line_str).unwrap();
        let maybe_start = extract_position(&captures, 0);
        let maybe_end = extract_position(&captures, 1);

        maybe_start.and_then(|start| maybe_end.map(|end| LineSegment { start, end }))
    }
}

fn parse_match_as_int(m: regex::Match) -> Result<i16, LineSegmentParseErr> {
    m.as_str().parse().or(Err("Unable to parse coordinate"))
}

fn extract_position(
    captures: &regex::Captures,
    position: u8,
) -> Result<Position, LineSegmentParseErr> {
    let maybe_x = captures
        .name(&format!("x{}", position))
        .ok_or("Unable to extract x coordinate")
        .and_then(parse_match_as_int);

    let maybe_y = captures
        .name(&format!("y{}", position))
        .ok_or("Unable to extract y coordinate")
        .and_then(parse_match_as_int);

    maybe_x.and_then(|x| maybe_y.map(|y| Position { x, y }))
}

fn build_intersections_map(lines: &Vec<&LineSegment>) -> Vec<Vec<u32>> {
    let max_x = LineSegment::max_coord_for_lines(lines, &Coordinate::X);
    let max_y = LineSegment::max_coord_for_lines(lines, &Coordinate::Y);

    let cols: usize = (max_x + 1).try_into().unwrap();
    let rows: usize = (max_y + 1).try_into().unwrap();

    let mut intersections_map = vec![vec![0; cols]; rows];
    for line in lines {
        let walk_list = line.build_walk_list();
        for pos in walk_list {
            let x: usize = pos.x.try_into().unwrap();
            let y: usize = pos.y.try_into().unwrap();
            intersections_map[y][x] += 1;
        }
    }

    intersections_map
}

fn _print_intersections_map(intersections_map: &Vec<Vec<u32>>) {
    let mut row_str = String::new();
    for row in intersections_map {
        row_str.clear();
        for col in row {
            row_str += &format!(
                "{} ",
                if *col == 0 {
                    String::from(".")
                } else {
                    col.to_string()
                }
            )
            .to_string();
        }
        println!("{}", row_str);
    }
}

fn count_overlapping_points(intersections_map: &Vec<Vec<u32>>) -> usize {
    intersections_map
        .into_iter()
        .flatten()
        .filter(|&n| n >= &2)
        .count()
}

pub struct P5;
impl Puzzle<Vec<LineSegment>> for P5 {
    fn number(&self) -> u8 {
        5
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<LineSegment> {
        raw_data
            .iter()
            .map(|line| LineSegment::from_str(line.as_str()).unwrap())
            .collect()
    }

    fn solve_part_one(&self, lines: &Vec<LineSegment>) {
        let vert_or_horiz_lines = lines
            .into_iter()
            .filter(|l| l.is_vertical_or_horizontal())
            .collect::<Vec<&LineSegment>>();
        let intersections_map = build_intersections_map(&vert_or_horiz_lines);
        let overlapping_points = count_overlapping_points(&intersections_map);
        println!("{:?}", overlapping_points);
    }

    fn solve_part_two(&self, _lines: &Vec<LineSegment>) {}
}
