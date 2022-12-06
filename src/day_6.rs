pub mod day_6 {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader, Read},
        str::from_utf8,
    };

    use crate::reader::reader::create_reader_from_file;

    pub fn day_6_part_1(reader: BufReader<File>) -> usize {
        let chars: Vec<(usize, String)> = reader
            .bytes()
            .map(|b| b.unwrap())
            .map(|b| from_utf8(&[b]).unwrap().to_owned())
            .enumerate()
            .collect();

        chars
            .as_slice()
            .windows(4)
            .filter(|w| {
                let tups: Vec<(String, usize)> = w
                    .iter()
                    .map(|(i, v)| (v.to_owned(), i.to_owned()))
                    .collect();
                let map: HashMap<String, usize> = HashMap::from_iter(tups);
                map.len() == 4
            })
            .take(1)
            .map(|v| v.into_iter().map(|(i, _)| i.to_owned()).max().unwrap())
            .nth(0)
            .unwrap()
            + 1
    }

    #[test]
    fn test_day_6_part_1_example() {
        let expect = 7;
        let got = day_6_part_1(create_reader_from_file(
            "./src/day_6/example.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_6_part_1_input() {
        let expect = 1766;
        let got = day_6_part_1(create_reader_from_file("./src/day_6/input.txt".to_owned()));
        assert_eq!(got, expect);
    }

    pub fn day_6_part_2(reader: BufReader<File>) -> usize {
        let chars: Vec<(usize, String)> = reader
            .bytes()
            .map(|b| b.unwrap())
            .map(|b| from_utf8(&[b]).unwrap().to_owned())
            .enumerate()
            .collect();

        chars
            .as_slice()
            .windows(14)
            .filter(|w| {
                let tups: Vec<(String, usize)> = w
                    .iter()
                    .map(|(i, v)| (v.to_owned(), i.to_owned()))
                    .collect();
                let map: HashMap<String, usize> = HashMap::from_iter(tups);
                map.len() == 14
            })
            .take(1)
            .map(|v| v.into_iter().map(|(i, _)| i.to_owned()).max().unwrap())
            .nth(0)
            .unwrap()
            + 1
    }

    #[test]
    fn test_day_6_part_2_example() {
        let expect = 19;

        let got = day_6_part_2(create_reader_from_file(
            "./src/day_6/example.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_6_part_2_input() {
        let expect = 2383;

        let got = day_6_part_2(create_reader_from_file("./src/day_6/input.txt".to_owned()));

        assert_eq!(got, expect);
    }
}
