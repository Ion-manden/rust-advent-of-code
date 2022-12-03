pub mod day_3 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::reader::reader::create_reader_from_file;

    fn get_item_priority(char: String) -> i32 {
        match char.as_str() {
            "a" => 1,
            "b" => 2,
            "c" => 3,
            "d" => 4,
            "e" => 5,
            "f" => 6,
            "g" => 7,
            "h" => 8,
            "i" => 9,
            "j" => 10,
            "k" => 11,
            "l" => 12,
            "m" => 13,
            "n" => 14,
            "o" => 15,
            "p" => 16,
            "q" => 17,
            "r" => 18,
            "s" => 19,
            "t" => 20,
            "u" => 21,
            "v" => 22,
            "w" => 23,
            "x" => 24,
            "y" => 25,
            "z" => 26,
            "A" => 27,
            "B" => 28,
            "C" => 29,
            "D" => 30,
            "E" => 31,
            "F" => 32,
            "G" => 33,
            "H" => 34,
            "I" => 35,
            "J" => 36,
            "K" => 37,
            "L" => 38,
            "M" => 39,
            "N" => 40,
            "O" => 41,
            "P" => 42,
            "Q" => 43,
            "R" => 44,
            "S" => 45,
            "T" => 46,
            "U" => 47,
            "V" => 48,
            "W" => 49,
            "X" => 50,
            "Y" => 51,
            "Z" => 52,
            _ => 0,
        }
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
            "./src/day_3/example.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_3_part_1_input() {
        let expect = 7908;
        let got = day_3_part_1(create_reader_from_file("./src/day_3/input.txt".to_owned()));
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
            "./src/day_3/example.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_3_part_2_input() {
        let expect = 2838;

        let got = day_3_part_2(create_reader_from_file("./src/day_3/input.txt".to_owned()));

        assert_eq!(got, expect);
    }
}
