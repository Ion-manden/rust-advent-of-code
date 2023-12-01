use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

use crate::reader::reader::create_reader_from_file;

#[derive(Debug, PartialEq)]
struct Monkey {
    pub id: usize,
    pub items: Vec<i64>,
    pub operation: String,
    pub test_divisible_by: i64,
    pub test_true_monkey: usize,
    pub test_false_monkey: usize,
    pub inspected_item_count: i64,
}

impl Monkey {
    // inspects item and returns new worry level
    pub fn inspect(&mut self, worry_level: &i64) -> i64 {
        self.inspected_item_count += 1;
        let parsed_operation = self
            .operation
            .replace("old", worry_level.to_string().as_str());

        let sections = parsed_operation.split(" ").collect_vec();
        let left: i64 = sections[0].parse().unwrap();
        let operation = sections[1];
        let right: i64 = sections[2].parse().unwrap();
        match operation {
            "*" => left * right,
            "+" => left + right,
            _ => panic!("Operation not implemented"),
        }
    }
}

fn map_monkey_chunk(chunk: Vec<String>) -> Monkey {
    // "Monkey 2:" => 2
    let id: usize = chunk[0]
        .trim()
        .replace("Monkey ", "")
        .replace(":", "")
        .parse()
        .unwrap();
    // Starting items
    // "  Starting items: 79, 60, 97" => [79,60,97]
    let items: Vec<i64> = chunk[1]
        .trim()
        .replace("Starting items: ", "")
        .split(", ")
        .map(|part| part.parse::<i64>().unwrap())
        .collect_vec();
    // "  Operation: new = old * old" => "old * old"
    let operation: String = chunk[2].trim().replace("Operation: new = ", "");
    // "  Test: divisible by 13" => 13
    let test_divisible_by: i64 = chunk[3]
        .trim()
        .replace("Test: divisible by ", "")
        .parse()
        .unwrap();
    // "    If true: throw to monkey 1" => 1
    let test_true_monkey: usize = chunk[4]
        .trim()
        .replace("If true: throw to monkey ", "")
        .parse()
        .unwrap();
    // "    If false: throw to monkey 3" => 3
    let test_false_monkey: usize = chunk[5]
        .trim()
        .replace("If false: throw to monkey ", "")
        .parse()
        .unwrap();

    return Monkey {
        id,
        items,
        operation,
        test_divisible_by,
        test_true_monkey,
        test_false_monkey,
        inspected_item_count: 0,
    };
}

#[test]
fn test_map_monkey_chunk() {
    let expect = Monkey {
        id: 2,
        items: vec![79, 60, 97],
        operation: "old * old".to_owned(),
        test_divisible_by: 13,
        test_true_monkey: 1,
        test_false_monkey: 3,
        inspected_item_count: 0,
    };
    let got = map_monkey_chunk(vec![
        String::from("Monkey 2:"),
        String::from("  Starting items: 79, 60, 97"),
        String::from("  Operation: new = old * old"),
        String::from("  Test: divisible by 13"),
        String::from("    If true: throw to monkey 1"),
        String::from("    If false: throw to monkey 3"),
        String::from(""),
    ]);

    assert_eq!(got, expect);
}

fn parse_monkeys(reader: BufReader<File>) -> Vec<Monkey> {
    let lines = reader.lines().map(|l| l.unwrap()).collect_vec();
    lines
        // Make chunks the length on one set of information about a monkey
        .chunks(7)
        .into_iter()
        .map(|c| {
            let chunk: Vec<String> = c.iter().map(|p| p.to_owned()).collect_vec();
            map_monkey_chunk(chunk)
        })
        .collect_vec()
}

pub fn day_11_part_1(reader: BufReader<File>) -> i64 {
    let mut monkeys = parse_monkeys(reader);

    for _ in 1..=20 {
        for monkey_index in 0..monkeys.len() {
            for item in monkeys[monkey_index].items.clone() {
                // We devide by 3 after inspection as we are relieved the item did not break during inspection
                let new_worry_level = monkeys[monkey_index].inspect(&item) / 3;

                if new_worry_level % monkeys[monkey_index].test_divisible_by == 0 {
                    let recieving_monkey_index = monkeys[monkey_index].test_true_monkey;
                    monkeys[recieving_monkey_index].items.push(new_worry_level);
                } else {
                    let recieving_monkey_index = monkeys[monkey_index].test_false_monkey;
                    monkeys[recieving_monkey_index].items.push(new_worry_level);
                }
            }

            // Reset monkeys inventory after round is over
            monkeys[monkey_index].items = Vec::new();
        }
    }

    let mut items_inspected = monkeys
        .into_iter()
        .map(|m| m.inspected_item_count)
        .collect_vec();
    items_inspected.sort();
    items_inspected.reverse();

    let top_two_inspection_counts = items_inspected.get(0..2).unwrap();

    top_two_inspection_counts[0] * top_two_inspection_counts[1]
}

#[test]
fn test_day_11_part_1_example() {
    let expect = 10605;
    let got = day_11_part_1(create_reader_from_file(
        "./src/solutions/day_11/example.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

#[test]
fn test_day_11_part_1_input() {
    let expect = 62491;
    let got = day_11_part_1(create_reader_from_file(
        "./src/solutions/day_11/input.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

pub fn day_11_part_2(reader: BufReader<File>) -> i64 {
    let mut monkeys = parse_monkeys(reader);

    let common_divider: i64 = monkeys.iter().map(|m| m.test_divisible_by).product();

    for _ in 1..=10_000 {
        for monkey_index in 0..monkeys.len() {
            for item in monkeys[monkey_index].items.clone() {
                // We mod the worry level by the divider to not get int overflows
                let new_worry_level = monkeys[monkey_index].inspect(&item) % common_divider ;

                if new_worry_level % monkeys[monkey_index].test_divisible_by == 0 {
                    let recieving_monkey_index = monkeys[monkey_index].test_true_monkey;
                    monkeys[recieving_monkey_index].items.push(new_worry_level);
                } else {
                    let recieving_monkey_index = monkeys[monkey_index].test_false_monkey;
                    monkeys[recieving_monkey_index].items.push(new_worry_level);
                }
            }

            // Reset monkeys inventory after round is over
            monkeys[monkey_index].items = Vec::new();
        }
    }

    let mut items_inspected = monkeys
        .into_iter()
        .map(|m| m.inspected_item_count)
        .collect_vec();

    items_inspected.sort();
    items_inspected.reverse();

    let top_two_inspection_counts = items_inspected.get(0..2).unwrap();

    top_two_inspection_counts[0] * top_two_inspection_counts[1]
}

#[test]
fn test_day_11_part_2_example() {
    let expect = 2713310158;

    let got = day_11_part_2(create_reader_from_file(
        "./src/solutions/day_11/example.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}

#[test]
fn test_day_11_part_2_input() {
    let expect = 17408399184;

    let got = day_11_part_2(create_reader_from_file(
        "./src/solutions/day_11/input.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}
