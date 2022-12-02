pub mod day_2 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::reader::reader::create_reader_from_file;

    enum Hand {
        Rock,
        Paper,
        Scicors,
    }

    fn get_hand_from_char(c: String) -> Result<Hand, String> {
        match c.as_str() {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scicors),
            "X" => Ok(Hand::Rock),
            "Y" => Ok(Hand::Paper),
            "Z" => Ok(Hand::Scicors),
            _ => Err("Invalid char".to_owned()),
        }
    }

    fn get_match_score(left: &Hand, right: &Hand) -> i32 {
        match (left, right) {
            (Hand::Rock, Hand::Rock) => 3,
            (Hand::Rock, Hand::Paper) => 6,
            (Hand::Rock, Hand::Scicors) => 0,
            (Hand::Paper, Hand::Rock) => 0,
            (Hand::Paper, Hand::Paper) => 3,
            (Hand::Paper, Hand::Scicors) => 6,
            (Hand::Scicors, Hand::Rock) => 6,
            (Hand::Scicors, Hand::Paper) => 0,
            (Hand::Scicors, Hand::Scicors) => 3,
        }
    }

    fn get_hand_for_outcome(left: &Hand, outcome: String) -> Result<Hand, String> {
        match (outcome.as_str(), left) {
            // Lose
            ("X", Hand::Rock) => Ok(Hand::Scicors),
            ("X", Hand::Paper) => Ok(Hand::Rock),
            ("X", Hand::Scicors) => Ok(Hand::Paper),
            // Tie
            ("Y", Hand::Rock) => Ok(Hand::Rock),
            ("Y", Hand::Paper) => Ok(Hand::Paper),
            ("Y", Hand::Scicors) => Ok(Hand::Scicors),
            // Win
            ("Z", Hand::Rock) => Ok(Hand::Paper),
            ("Z", Hand::Paper) => Ok(Hand::Scicors),
            ("Z", Hand::Scicors) => Ok(Hand::Rock),
            _ => Err("Invalid condition".to_owned()),
        }
    }

    fn get_hand_score(h: Hand) -> i32 {
        match h {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scicors => 3,
        }
    }

    pub fn day_2_part_1(reader: BufReader<File>) -> i32 {
        let mut score: i32 = 0;

        for line in reader.lines() {
            if let Ok(l) = line {
                let mut p = l.split(" ").into_iter();

                let pl = p.nth(0).unwrap().to_owned();
                let left = get_hand_from_char(pl).unwrap();
                let pr = p.nth(0).unwrap().to_owned();
                let right = get_hand_from_char(pr).unwrap();

                score += get_match_score(&left, &right) + get_hand_score(right);
            }
        }

        score
    }

    #[test]
    fn test_day_2_part_1_example() {
        let expect = 15;
        let got = day_2_part_1(create_reader_from_file(
            "./src/day_2/example.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_2_part_1_input() {
        let expect = 10404;
        let got = day_2_part_1(create_reader_from_file("./src/day_2/input.txt".to_owned()));
        assert_eq!(got, expect);
    }

    pub fn day_2_part_2(reader: BufReader<File>) -> i32 {
        let mut score: i32 = 0;

        for line in reader.lines() {
            if let Ok(l) = line {
                let mut p = l.split(" ").into_iter();

                let pl = p.nth(0).unwrap().to_owned();
                let left = get_hand_from_char(pl).unwrap();
                let how_round_should_end = p.nth(0).unwrap().to_owned();
                let right = get_hand_for_outcome(&left, how_round_should_end).unwrap();

                score += get_match_score(&left, &right) + get_hand_score(right);
            }
        }

        score
    }

    #[test]
    fn test_day_2_part_2_example() {
        let expect = 12;

        let got = day_2_part_2(create_reader_from_file(
            "./src/day_2/example.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_2_part_2_input() {
        let expect = 10334;

        let got = day_2_part_2(create_reader_from_file("./src/day_2/input.txt".to_owned()));

        assert_eq!(got, expect);
    }
}
