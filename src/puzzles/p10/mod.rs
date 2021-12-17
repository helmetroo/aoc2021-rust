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

fn matching_closed(chr: char) -> char {
    match chr {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
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

fn close_score(chr: char) -> u64 {
    match chr {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn score_program(program: &Program) -> u32 {
    program
        .iter()
        .map(|line| score(find_first_incorrect_close_chr(&line)))
        .sum()
}

fn get_completion_str(line: &String) -> Vec<char> {
    let mut stack = Vec::new();
    let mut closing_chars = Vec::new();

    for chr in line.chars() {
        if is_open(chr) {
            stack.push(chr);
        } else {
            let top = stack[stack.len() - 1];
            if top == matching_open(chr) {
                stack.pop();
            }
        }
    }

    while !stack.is_empty() {
        let top = stack[stack.len() - 1];
        let as_closed = matching_closed(top);
        closing_chars.push(as_closed);
        stack.pop();
    }

    closing_chars
}

fn score_completion_str(completion_str: &Vec<char>) -> u64 {
    completion_str
        .iter()
        .fold(0, |score, &chr| (score * 5) + close_score(chr))
}

fn middle_score(program: &Program) -> u64 {
    let mut scores = program
        .iter()
        .filter(|line| find_first_incorrect_close_chr(*line).is_none())
        .map(|line| score_completion_str(&get_completion_str(line)))
        .collect::<Vec<u64>>();

    scores.sort();

    let middle_index = scores.len() / 2;
    scores[middle_index]
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

    fn solve_part_two(&self, program: &Program) {
        let score = middle_score(program);
        println!("{}", score);
    }
}
