pub mod day_3 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::reader::reader::create_reader_from_file;

    fn get_item_priority(char: String) -> i32 {
        let i = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .position(|c| c.to_string() == char)
            .unwrap();
        (i + 1) as i32
    }

    pub fn day_3_part_1(reader: BufReader<File>) -> i32 {
        let mut sum: i32 = 0;

        for line in reader.lines() {
            if let Ok(l) = line {
                let chars = l.chars();
                let c_length = chars.clone().count();
                let first_compartment = chars.clone().take(c_length / 2);
                let second_compartment = chars.clone().rev().take(c_length / 2);

                let common_char = first_compartment
                    .to_owned()
                    .find(|fc| second_compartment.to_owned().any(|sc| fc == &sc));

                if let Some(c) = common_char {
                    sum += get_item_priority(c.to_string());
                }
            }
        }

        sum
    }

    #[test]
    fn test_day_3_part_1_example() {
        let expect = 157;
        let got = day_3_part_1(create_reader_from_file(
            "./src/solutions/day_3/example.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_3_part_1_input() {
        let expect = 7908;
        let got = day_3_part_1(create_reader_from_file("./src/solutions/day_3/input.txt".to_owned()));
        assert_eq!(got, expect);
    }

    pub fn day_3_part_2(reader: BufReader<File>) -> i32 {
        let mut sum: i32 = 0;

        let mut lines = reader.lines();
        while let (Some(line1), Some(line2), Some(line3)) =
            (lines.next(), lines.next(), lines.next())
        {
            let l1 = line1.unwrap();
            let l2 = line2.unwrap();
            let l3 = line3.unwrap();

            let first_elf = l1.chars();
            let second_elf = l2.chars();
            let third_elf = l3.chars();

            let first_and_second_common_chars = first_elf
                .clone()
                .filter(|fc| second_elf.clone().to_owned().any(|sc| fc == &sc));

            let common_char = first_and_second_common_chars
                .clone()
                .find(|c| third_elf.clone().any(|tc| c == &tc));

            if let Some(c) = common_char {
                sum += get_item_priority(c.to_string());
            }
        }

        sum
    }

    #[test]
    fn test_day_3_part_2_example() {
        let expect = 70;

        let got = day_3_part_2(create_reader_from_file(
            "./src/solutions/day_3/example.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_3_part_2_input() {
        let expect = 2838;

        let got = day_3_part_2(create_reader_from_file("./src/solutions/day_3/input.txt".to_owned()));

        assert_eq!(got, expect);
    }
}
