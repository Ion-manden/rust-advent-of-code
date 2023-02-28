use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::reader::reader::create_reader_from_file;

pub fn day_10_part_1(reader: BufReader<File>) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut signal_strength_mesurements: Vec<i32> = Vec::new();

    for l in reader.lines() {
        let line = l.unwrap();

        if line == "noop" {
            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                signal_strength_mesurements.push(cycle * x);
            }
            continue;
        }

        let (_instruction, value) = line.split_once(" ").unwrap();
        let n: i32 = value.parse::<i32>().unwrap();

        cycle += 1;
        if (cycle + 20) % 40 == 0 {
            signal_strength_mesurements.push(cycle * x);
        }
        cycle += 1;
        if (cycle + 20) % 40 == 0 {
            signal_strength_mesurements.push(cycle * x);
        }

        x = x + n;
    }

    signal_strength_mesurements.into_iter().sum()
}

#[test]
fn test_day_10_part_1_example() {
    let expect = 13140;
    let got = day_10_part_1(create_reader_from_file(
        "./src/solutions/day_10/example.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

#[test]
fn test_day_10_part_1_input() {
    let expect = 13740;
    let got = day_10_part_1(create_reader_from_file(
        "./src/solutions/day_10/input.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

fn get_lit_pixels(x: &i32) -> Vec<i32> {
    let mut lit_pixels: Vec<i32> = Vec::new();

    let left_postion = x % 40;
    lit_pixels.push(left_postion);
    let middle_postion = (x % 40) + 1;
    lit_pixels.push(middle_postion % 40);
    let right_postion = (x % 40) + 2;
    lit_pixels.push(right_postion % 40);

    lit_pixels
}
#[test]
fn test_get_lit_pixels() {
    {
        let expect: Vec<i32> = vec![30, 31, 32];
        let got = get_lit_pixels(&30);
        assert_eq!(got, expect);
    }
    {
        let expect: Vec<i32> = vec![38, 39, 0];
        let got = get_lit_pixels(&38);
        assert_eq!(got, expect);
    }
}

pub fn day_10_part_2(reader: BufReader<File>) -> Vec<String> {
    let mut cycle = 0;
    let mut x = 1;
    let mut terminal_output: Vec<String> = Vec::new();

    for l in reader.lines() {
        let line = l.unwrap();

        if line == "noop" {
            cycle += 1;
            if get_lit_pixels(&x).contains(&(cycle % 40)) {
                terminal_output.push("#".to_owned());
            } else {
                terminal_output.push(".".to_owned());
            }
            continue;
        }

        let (_instruction, value) = line.split_once(" ").unwrap();
        let n: i32 = value.parse::<i32>().unwrap();

        cycle += 1;
        if get_lit_pixels(&x).contains(&(cycle % 40)) {
            terminal_output.push("#".to_owned());
        } else {
            terminal_output.push(".".to_owned());
        }
        cycle += 1;
        if get_lit_pixels(&x).contains(&(cycle % 40)) {
            terminal_output.push("#".to_owned());
        } else {
            terminal_output.push(".".to_owned());
        }

        x = x + n;
    }

    for i in 0..terminal_output.len() {
        print!("{}", terminal_output[i]);
        if (i + 1) % 40 == 0 {
            print!("\n");
        }
    }

    terminal_output
}

#[test]
fn test_day_10_part_2_example() {
    let expect = vec![
        "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#",
        ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".", "#", "#", ".", ".",
        "#", "#", ".", ".", "#", "#", "#", ".", ".", ".", "#", "#", "#", ".", ".", ".", "#", "#",
        "#", ".", ".", ".", "#", "#", "#", ".", ".", ".", "#", "#", "#", ".", ".", ".", "#", "#",
        "#", ".", ".", ".", "#", "#", "#", ".", "#", "#", "#", "#", ".", ".", ".", ".", "#", "#",
        "#", "#", ".", ".", ".", ".", "#", "#", "#", "#", ".", ".", ".", ".", "#", "#", "#", "#",
        ".", ".", ".", ".", "#", "#", "#", "#", ".", ".", ".", ".", "#", "#", "#", "#", "#", ".",
        ".", ".", ".", ".", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", "#", "#", "#", "#",
        "#", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", "#", "#",
        "#", "#", "#", "#", ".", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", "#", ".", ".",
        ".", ".", ".", ".", "#", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", ".", "#", "#",
        "#", "#", "#", "#", "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", ".", ".", "#", "#",
        "#", "#", "#", "#", "#", ".", ".", ".", ".", ".", ".", ".", "#", "#", "#", "#", "#", "#",
        "#", ".", ".", ".", ".", ".",
    ];

    let got = day_10_part_2(create_reader_from_file(
        "./src/solutions/day_10/example.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}

#[test]
fn test_day_10_part_2_input() {
    let expect = vec![
        "#", "#", "#", "#", ".", "#", ".", ".", "#", ".", "#", "#", "#", ".", ".", "#", "#", "#",
        ".", ".", "#", "#", "#", "#", ".", "#", "#", "#", "#", ".", ".", "#", "#", ".", ".", "#",
        ".", ".", ".", ".", ".", ".", ".", "#", ".", "#", ".", ".", "#", ".", "#", ".", ".", "#",
        ".", "#", ".", ".", "#", ".", "#", ".", ".", ".", ".", "#", ".", ".", ".", ".", "#", ".",
        ".", "#", ".", "#", ".", ".", ".", ".", ".", ".", "#", ".", ".", "#", ".", ".", "#", ".",
        "#", ".", ".", "#", ".", "#", ".", ".", "#", ".", "#", "#", "#", ".", ".", "#", "#", "#",
        ".", ".", "#", ".", ".", ".", ".", "#", ".", ".", ".", ".", ".", "#", ".", ".", ".", "#",
        ".", ".", "#", ".", "#", "#", "#", ".", ".", "#", "#", "#", ".", ".", "#", ".", ".", ".",
        ".", "#", ".", ".", ".", ".", "#", ".", ".", ".", ".", "#", ".", ".", ".", ".", "#", ".",
        ".", ".", ".", "#", ".", ".", "#", ".", "#", ".", ".", ".", ".", "#", ".", "#", ".", ".",
        "#", ".", ".", ".", ".", "#", ".", ".", ".", ".", "#", ".", ".", "#", ".", "#", ".", ".",
        ".", "#", "#", "#", "#", "#", ".", ".", "#", "#", ".", ".", "#", ".", ".", ".", ".", "#",
        ".", ".", "#", ".", "#", ".", ".", ".", ".", "#", "#", "#", "#", ".", ".", "#", "#", ".",
        ".", "#", "#", "#", "#", ".",
    ];

    let got = day_10_part_2(create_reader_from_file(
        "./src/solutions/day_10/input.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}
