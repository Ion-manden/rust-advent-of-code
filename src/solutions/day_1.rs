
pub mod day_1 {
    use std::{io::{BufReader, BufRead}, fs::File};

    use crate::reader::reader::create_reader_from_file;

    pub fn day_1_part_1(reader: BufReader<File>) -> i32 {
        let mut highest = 0;
        let mut sum: i32 = 0;

        for line in reader.lines() {
            if let Ok(l) = line {
                if l == "" {
                    if sum > highest {
                        highest = sum;
                    }
                    sum = 0;
                    continue;
                }

                let n: i32 = l.parse().unwrap();

                sum += n;
            }
        }
        if sum > highest {
            highest = sum;
        }

        highest
    }

    #[test]
    fn test_day_1_part_1_example() {
        let expect = 24000;
        let got = day_1_part_1(create_reader_from_file(
            "./src/solutions/day_1/example.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_1_part_1_input() {
        let expect = 69912;
        let got = day_1_part_1(create_reader_from_file(
            "./src/solutions/day_1/input.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    pub fn day_1_part_2(reader: BufReader<File>) -> i32 {
        let mut sums: Vec<i32> = Vec::new();
        let mut sum: i32 = 0;

        for line in reader.lines() {
            if let Ok(l) = line {
                if l == "" {
                    sums.push(sum);
                    sum = 0;
                    continue;
                }

                let n: i32 = l.parse().unwrap();

                sum += n;
            }
        }
        sums.push(sum);

        sums.sort_unstable();
        sums.reverse();
        sums.truncate(3);
        sums.into_iter().sum()
    }

    #[test]
    fn test_day_1_part_2_example() {
        let expect = 45000;

        let got = day_1_part_2(create_reader_from_file(
            "./src/solutions/day_1/example.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_1_part_2_input() {
        let expect = 208180;

        let got = day_1_part_2(create_reader_from_file(
            "./src/solutions/day_1/input.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }
}
