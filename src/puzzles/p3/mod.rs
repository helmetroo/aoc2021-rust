use crate::puzzles::puzzle::Puzzle;
use crate::utils::input_file;

pub struct Rates {
    gamma: u32,
    epsilon: u32,
}

impl Rates {
    pub fn new() -> Self {
        Self { gamma: 0u32, epsilon: 0u32 }
    }

    pub fn multiply_values(&self) -> u32 {
        self.gamma * self.epsilon
    }
}

fn power_consumption(report: &Vec<Vec<char>>) -> u32 {
    let bit_width = report[0].len();
    let report_size = report.len();

    (0..bit_width).fold(Rates::new(), |rates, bit| {
        let (ones, zeroes) = count_bits_at(bit, report_size, report);
        let bit_to_set = bit_width - bit - 1;
        let value_to_merge = 1 << bit_to_set;
        if ones >= zeroes {
            Rates { gamma: rates.gamma | value_to_merge, ..rates }
        } else {
            Rates { epsilon: rates.epsilon | value_to_merge, ..rates }
        }
    }).multiply_values()
}

fn life_support_rating(report: &Vec<Vec<char>>) -> u32 {
    let bit_width = report[0].len();

    let oxygen_rating = get_rating(report, true, bit_width);
    let co2_rating = get_rating(report, false, bit_width);

    oxygen_rating * co2_rating
}

fn get_rating(report: &Vec<Vec<char>>, for_oxygen: bool, bit_width: usize) -> u32 {
    let mut kept_nums = report.clone();
    for bit in 0..bit_width {
        let (ones, zeroes) = count_bits_at(bit, kept_nums.len(), &kept_nums);
        kept_nums.retain(|n: &Vec<char>| {
            let keep_one = if for_oxygen {
                ones >= zeroes
            } else {
                ones < zeroes
            };
            keep_one && n[bit] == '1' || !keep_one && n[bit] == '0'
        });

        if kept_nums.len() == 1 {
            break;
        }
    }

    build_number_from_chars(&kept_nums[0])
}

fn build_number_from_chars(chars: &Vec<char>) -> u32 {
    let bit_width = chars.len();
    (0..bit_width).fold(0u32, |number, bit| {
        let bit_to_set = bit_width - bit - 1;
        if chars[bit] == '1' {
            let value_to_merge = 1 << bit_to_set;
            number | value_to_merge
        } else {
            number
        }
    })
}

fn count_bits_at(bit: usize, report_size: usize, report: &Vec<Vec<char>>) -> (usize, usize) {
    let ones = report.iter().fold(0, |acc, number| {
        let bit_value = number[bit];
        if bit_value == '1' {
            acc + 1
        } else {
            acc
        }
    });

    let zeroes = report_size - ones;
    (ones, zeroes)
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

    fn solve_part_two(&self, report: &Vec<Vec<char>>) {
        println!("{}", life_support_rating(report))
    }
}
