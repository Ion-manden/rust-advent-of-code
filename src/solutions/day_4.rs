pub mod day_4 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::reader::reader::create_reader_from_file;

    struct Elf {
        lower: i32,
        upper: i32,
    }

    fn assign_elf(assignment: String) -> Elf {
        let edges = assignment.split("-").collect::<Vec<_>>();

        let lower = edges[0].parse::<i32>().unwrap();
        let upper = edges[1].parse::<i32>().unwrap();

        Elf { lower, upper }
    }

    fn get_elfs_from_assign_pair(assignment_pair: String) -> (Elf, Elf) {
        let assignments = assignment_pair.split(",").collect::<Vec<_>>();
        let elf1 = assign_elf(assignments[0].to_owned());
        let elf2 = assign_elf(assignments[1].to_owned());

        (elf1, elf2)
    }

    #[test]
    fn test_assign_elf() {
        let (elf1, elf2) = get_elfs_from_assign_pair("5-7,7-9".to_owned());

        let expected_elf1 = Elf { lower: 5, upper: 7 };
        let expected_elf2 = Elf { lower: 7, upper: 9 };
        assert_eq!(elf1.lower, expected_elf1.lower);
        assert_eq!(elf1.upper, expected_elf1.upper);
        assert_eq!(elf2.lower, expected_elf2.lower);
        assert_eq!(elf1.upper, expected_elf1.upper);
    }

    pub fn day_4_part_1(reader: BufReader<File>) -> i32 {
        reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| -> i32 {
                let (elf1, elf2) = get_elfs_from_assign_pair(line);

                if elf1.lower <= elf2.lower && elf1.upper >= elf2.upper {
                    return 1;
                }

                if elf2.lower <= elf1.lower && elf2.upper >= elf1.upper {
                    return 1;
                }

                return 0;
            })
            .sum()
    }

    #[test]
    fn test_day_4_part_1_example() {
        let expect = 2;
        let got = day_4_part_1(create_reader_from_file(
            "./src/solutions/day_4/example.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_4_part_1_input() {
        let expect = 450;
        let got = day_4_part_1(create_reader_from_file("./src/solutions/day_4/input.txt".to_owned()));
        assert_eq!(got, expect);
    }

    pub fn day_4_part_2(reader: BufReader<File>) -> i32 {
        reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| -> i32 {
                let (elf1, elf2) = get_elfs_from_assign_pair(line);

                if elf1.lower <= elf2.upper && elf1.upper >= elf2.lower {
                    return 1;
                }

                if elf2.lower <= elf1.upper && elf2.upper >= elf1.lower {
                    return 1;
                }

                return 0;
            })
            .sum()
    }

    #[test]
    fn test_day_4_part_2_example() {
        let expect = 4;

        let got = day_4_part_2(create_reader_from_file(
            "./src/solutions/day_4/example.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_4_part_2_input() {
        let expect = 837;

        let got = day_4_part_2(create_reader_from_file("./src/solutions/day_4/input.txt".to_owned()));

        assert_eq!(got, expect);
    }
}
