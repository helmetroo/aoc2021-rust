use crate::utils::input_file;

pub trait Puzzle<T> {
    fn number(&self) -> u8;
    fn solve_part_one(&self, data: &Vec<T>);
    fn solve_part_two(&self, data: &Vec<T>);

    fn read_input_file(&self, test_input: bool) -> Vec<String> {
        let number = self.number();
        input_file::read(number, test_input)
    }

    fn parse_data(&self, raw_data: &Vec<String>) -> Vec<T>;

    fn solve(&self, test_input: bool) {
        let raw_data = self.read_input_file(test_input);
        let data = self.parse_data(&raw_data);

        self.solve_part_one(&data);
        self.solve_part_two(&data);
    }
}
