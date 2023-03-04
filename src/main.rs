mod reader;
mod solutions;

use std::{fmt::Display, time::Instant};

use reader::reader::create_reader_from_file;
use solutions::day_1::day_1::{day_1_part_1, day_1_part_2};
use solutions::day_2::day_2::{day_2_part_1, day_2_part_2};
use solutions::day_3::day_3::{day_3_part_1, day_3_part_2};
use solutions::day_4::day_4::{day_4_part_1, day_4_part_2};
use solutions::day_5::day_5::{day_5_part_1, day_5_part_2};
use solutions::day_6::day_6::{day_6_part_1, day_6_part_2};
use solutions::day_7::day_7::{day_7_part_1, day_7_part_2};
use solutions::day_8::day_8::{day_8_part_1, day_8_part_2};
use solutions::day_9::day_9::{day_9_part_1, day_9_part_2};
use solutions::day_10::day_10::{day_10_part_1, day_10_part_2};
use solutions::day_11::day_11::{day_11_part_1, day_11_part_2};
use solutions::day_12::day_12::{day_12_part_1, day_12_part_2};

fn print_metric<T: Display>(name: String, func: fn() -> T) {
    println!("{}", name);

    let timer = Instant::now();
    let res = func();
    println!("Got: {}", res);
    let time = timer.elapsed();
    println!("took: {:.2?}", time);

    println!("");
}

fn main() {
    print_metric(String::from("Day 1 Part 1"), || {
        day_1_part_1(create_reader_from_file(
            "./src/solutions/day_1/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 1 Part 2"), || {
        day_1_part_2(create_reader_from_file(
            "./src/solutions/day_1/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 2 Part 1"), || {
        day_2_part_1(create_reader_from_file(
            "./src/solutions/day_2/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 2 Part 2"), || {
        day_2_part_2(create_reader_from_file(
            "./src/solutions/day_2/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 3 Part 1"), || {
        day_3_part_1(create_reader_from_file(
            "./src/solutions/day_3/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 3 Part 2"), || {
        day_3_part_2(create_reader_from_file(
            "./src/solutions/day_3/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 4 Part 1"), || {
        day_4_part_1(create_reader_from_file(
            "./src/solutions/day_4/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 4 Part 2"), || {
        day_4_part_2(create_reader_from_file(
            "./src/solutions/day_4/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 5 Part 1"), || {
        day_5_part_1(create_reader_from_file(
            "./src/solutions/day_5/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 5 Part 2"), || {
        day_5_part_2(create_reader_from_file(
            "./src/solutions/day_5/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 6 Part 1"), || {
        day_6_part_1(create_reader_from_file(
            "./src/solutions/day_6/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 6 Part 2"), || {
        day_6_part_2(create_reader_from_file(
            "./src/solutions/day_6/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 7 Part 1"), || {
        day_7_part_1(create_reader_from_file(
            "./src/solutions/day_7/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 7 Part 2"), || {
        day_7_part_2(create_reader_from_file(
            "./src/solutions/day_7/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 8 Part 1"), || {
        day_8_part_1(create_reader_from_file(
            "./src/solutions/day_8/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("Day 8 Part 2"), || {
        day_8_part_2(create_reader_from_file(
            "./src/solutions/day_8/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("day 9 part 1"), || {
        day_9_part_1(create_reader_from_file(
            "./src/solutions/day_9/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("day 9 part 2"), || {
        day_9_part_2(create_reader_from_file(
            "./src/solutions/day_9/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("day 10 part 1"), || {
        day_10_part_1(create_reader_from_file(
            "./src/solutions/day_10/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("day 10 part 2"), || {
        day_10_part_2(create_reader_from_file(
            "./src/solutions/day_10/input.txt".to_owned(),
        )).len()
    });
    print_metric(String::from("day 11 part 1"), || {
        day_11_part_1(create_reader_from_file(
            "./src/solutions/day_11/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("day 11 part 2"), || {
        day_11_part_2(create_reader_from_file(
            "./src/solutions/day_11/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("day 12 part 1"), || {
        day_12_part_1(create_reader_from_file(
            "./src/solutions/day_12/input.txt".to_owned(),
        ))
    });
    print_metric(String::from("day 12 part 2"), || {
        day_12_part_2(create_reader_from_file(
            "./src/solutions/day_12/input.txt".to_owned(),
        ))
    });
}
