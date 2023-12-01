pub mod day_1;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

fn print_metric<T: Display>(name: String, func: fn() -> T) {
    println!("{}", name);

    let timer = Instant::now();
    let res = func();
    println!("Got: {}", res);
    let time = timer.elapsed();
    println!("took: {:.2?}", time);

    println!("");
}

pub fn solve() {
    print_metric(String::from("Day 1 Part 1"), || {
        day_1_part_1(create_reader_from_file("./day_1/input.txt".to_owned()))
    });
    print_metric(String::from("Day 1 Part 2"), || {
        day_1_part_2(create_reader_from_file("./day_1/input.txt".to_owned()))
    });
    print_metric(String::from("Day 2 Part 1"), || {
        day_2_part_1(create_reader_from_file("./day_2/input.txt".to_owned()))
    });
    print_metric(String::from("Day 2 Part 2"), || {
        day_2_part_2(create_reader_from_file("./day_2/input.txt".to_owned()))
    });
    print_metric(String::from("Day 3 Part 1"), || {
        day_3_part_1(create_reader_from_file("./day_3/input.txt".to_owned()))
    });
    print_metric(String::from("Day 3 Part 2"), || {
        day_3_part_2(create_reader_from_file("./day_3/input.txt".to_owned()))
    });
    print_metric(String::from("Day 4 Part 1"), || {
        day_4_part_1(create_reader_from_file("./day_4/input.txt".to_owned()))
    });
    print_metric(String::from("Day 4 Part 2"), || {
        day_4_part_2(create_reader_from_file("./day_4/input.txt".to_owned()))
    });
    print_metric(String::from("Day 5 Part 1"), || {
        day_5_part_1(create_reader_from_file("./day_5/input.txt".to_owned()))
    });
    print_metric(String::from("Day 5 Part 2"), || {
        day_5_part_2(create_reader_from_file("./day_5/input.txt".to_owned()))
    });
    print_metric(String::from("Day 6 Part 1"), || {
        day_6_part_1(create_reader_from_file("./day_6/input.txt".to_owned()))
    });
    print_metric(String::from("Day 6 Part 2"), || {
        day_6_part_2(create_reader_from_file("./day_6/input.txt".to_owned()))
    });
    print_metric(String::from("Day 7 Part 1"), || {
        day_7_part_1(create_reader_from_file("./day_7/input.txt".to_owned()))
    });
    print_metric(String::from("Day 7 Part 2"), || {
        day_7_part_2(create_reader_from_file("./day_7/input.txt".to_owned()))
    });
    print_metric(String::from("Day 8 Part 1"), || {
        day_8_part_1(create_reader_from_file("./day_8/input.txt".to_owned()))
    });
    print_metric(String::from("Day 8 Part 2"), || {
        day_8_part_2(create_reader_from_file("./day_8/input.txt".to_owned()))
    });
    print_metric(String::from("day 9 part 1"), || {
        day_9_part_1(create_reader_from_file("./day_9/input.txt".to_owned()))
    });
    print_metric(String::from("day 9 part 2"), || {
        day_9_part_2(create_reader_from_file("./day_9/input.txt".to_owned()))
    });
    print_metric(String::from("day 10 part 1"), || {
        day_10_part_1(create_reader_from_file("./day_10/input.txt".to_owned()))
    });
    print_metric(String::from("day 10 part 2"), || {
        day_10_part_2(create_reader_from_file("./day_10/input.txt".to_owned())).len()
    });
    print_metric(String::from("day 11 part 1"), || {
        day_11_part_1(create_reader_from_file("./day_11/input.txt".to_owned()))
    });
    print_metric(String::from("day 11 part 2"), || {
        day_11_part_2(create_reader_from_file("./day_11/input.txt".to_owned()))
    });
    print_metric(String::from("day 12 part 1"), || {
        day_12_part_1(create_reader_from_file("./day_12/input.txt".to_owned()))
    });
    print_metric(String::from("day 12 part 2"), || {
        day_12_part_2(create_reader_from_file("./day_12/input.txt".to_owned()))
    });
    print_metric(String::from("day 13 part 1"), || {
        day_13_part_1(create_reader_from_file("./day_13/input.txt".to_owned()))
    });
    print_metric(String::from("day 13 part 2"), || {
        day_13_part_2(create_reader_from_file("./day_13/input.txt".to_owned()))
    });
    print_metric(String::from("day 14 part 1"), || {
        day_14_part_1(create_reader_from_file("./day_14/input.txt".to_owned()))
    });
    print_metric(String::from("day 14 part 2"), || {
        day_14_part_2(create_reader_from_file("./day_14/input.txt".to_owned()))
    });
    print_metric(String::from("day 15 part 1"), || {
        day_15_part_1(
            create_reader_from_file("./day_15/input.txt".to_owned()),
            200_0000,
        )
    });
    print_metric(String::from("day 15 part 2"), || {
        day_15_part_2(create_reader_from_file("./day_15/input.txt".to_owned()))
    });
}
