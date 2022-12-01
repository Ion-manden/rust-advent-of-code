mod day_1;
mod reader;

use day_1::day_1::{day_1_part_1, day_1_part_2};
use reader::reader::create_reader_from_file;

fn main() {
    println!("{}", day_1_part_1(create_reader_from_file("./src/day_1/input.txt".to_owned())));
    println!("{}", day_1_part_2(create_reader_from_file("./src/day_1/input.txt".to_owned())));
}

