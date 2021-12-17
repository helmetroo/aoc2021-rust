use crate::puzzles::puzzle::Puzzle;

type Program = Vec<String>;

fn find_first_incorrect_close_chr(line: &String) -> Option<char> {
    let mut stack = Vec::new();
    line.chars().find(|&chr| {
        if is_open(chr) {
            stack.push(chr);
            false
        } else {
            let top = stack[stack.len() - 1];
            if top == matching_open(chr) {
                stack.pop();
                false
            } else {
                true
            }
        }
    })
}

fn is_open(chr: char) -> bool {
    chr == '{' || chr == '(' || chr == '[' || chr == '<'
}

fn matching_open(chr: char) -> char {
    match chr {
        '}' => '{',
        ')' => '(',
        ']' => '[',
        '>' => '<',
        _ => chr,
    }
}

fn score(maybe_chr: Option<char>) -> u32 {
    if let Some(chr) = maybe_chr {
        match chr {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    } else {
        0
    }
}

fn score_program(program: &Program) -> u32 {
    program
        .iter()
        .map(|line| score(find_first_incorrect_close_chr(&line)))
        .sum()
}

pub struct P10;
impl Puzzle<Program> for P10 {
    fn number(&self) -> u8 {
        10
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Program {
        raw_data.clone()
    }

    fn solve_part_one(&self, program: &Program) {
        let score = score_program(program);
        println!("{}", score);
    }

    fn solve_part_two(&self, _program: &Program) {}
}
