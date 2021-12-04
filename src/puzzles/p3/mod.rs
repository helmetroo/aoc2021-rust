use crate::puzzles::puzzle::Puzzle;
use crate::utils::input_file;

fn power_consumption(report: &Vec<Vec<char>>) -> u32 {
    let mut gamma_rate = 0u32;
    let mut epsilon_rate = 0u32;

    let bit_width = report[0].len();
    let report_size = report.len();

    for bit in 0..bit_width {
        let ones_counted = report.iter().fold(0, |acc, number| {
            let bit_value = number[bit];
            if bit_value == '1' {
                acc + 1
            } else {
                acc
            }
        });

        let zeros_counted = report_size - ones_counted;
        let bit_to_set = bit_width - bit - 1;
        if ones_counted >= zeros_counted {
            gamma_rate |= 1 << bit_to_set
        } else {
            epsilon_rate |= 1 << bit_to_set
        }
    }

    gamma_rate * epsilon_rate
}

pub struct P3;
impl Puzzle<Vec<char>> for P3 {
    fn number(&self) -> u8 {
        3
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<Vec<char>> {
        raw_data.iter().map(input_file::as_chars).collect()
    }

    fn solve_part_one(&self, report: &Vec<Vec<char>>) {
        println!("{}", power_consumption(report))
    }

    fn solve_part_two(&self, _report: &Vec<Vec<char>>) {}
}
