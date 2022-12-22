pub mod day_5 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use regex::Regex;

    use crate::reader::reader::create_reader_from_file;

    type Stack = Vec<Vec<String>>;

    fn parse_line_as_sections(line: String) -> Vec<String> {
        line.chars()
            .enumerate()
            .filter(|(i, _)| i % 4 == 1)
            .map(|(_, c)| String::from(c))
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_parse_line_as_sections() {
        let expect = Vec::from([String::from("N"), String::from("C"), String::from(" ")]);
        let got = parse_line_as_sections("[N] [C]    ".to_owned());
        assert_eq!(got, expect);
    }

    fn move_stack(operation: String, mut stack: Stack) -> Stack {
        let re = Regex::new(r" \d+").unwrap();
        let mut matches = re.find_iter(&operation);

        let n_to_move = matches
            .nth(0)
            .unwrap()
            .as_str()
            .replace(" ", "")
            .parse::<i32>()
            .unwrap();
        let from_stack = matches
            .nth(0)
            .unwrap()
            .as_str()
            .replace(" ", "")
            .parse::<usize>()
            .unwrap()
            - 1;
        let to_stack = matches
            .nth(0)
            .unwrap()
            .as_str()
            .replace(" ", "")
            .parse::<usize>()
            .unwrap()
            - 1;

        for _ in 0..n_to_move {
            let item = stack.get_mut(from_stack).unwrap().pop().unwrap();
            stack.get_mut(to_stack).unwrap().push(item);
        }

        stack
    }

    #[test]
    fn test_move_stack() {
        let expect = Vec::from([
            Vec::from([String::from("Z"), String::from("N"), String::from("D")]),
            Vec::from([String::from("M"), String::from("C")]),
            Vec::from([String::from("P")]),
        ]);
        let got = move_stack(
            "move 1 from 2 to 1".to_owned(),
            Vec::from([
                Vec::from([String::from("Z"), String::from("N")]),
                Vec::from([String::from("M"), String::from("C"), String::from("D")]),
                Vec::from([String::from("P")]),
            ]),
        );
        assert_eq!(got, expect);
    }

    fn move_stack_batch(operation: String, mut stack: Stack) -> Stack {
        let re = Regex::new(r" \d+").unwrap();
        let mut matches = re.find_iter(&operation);

        let n_to_move = matches
            .nth(0)
            .unwrap()
            .as_str()
            .replace(" ", "")
            .parse::<usize>()
            .unwrap();
        let from_stack = matches
            .nth(0)
            .unwrap()
            .as_str()
            .replace(" ", "")
            .parse::<usize>()
            .unwrap()
            - 1;
        let to_stack = matches
            .nth(0)
            .unwrap()
            .as_str()
            .replace(" ", "")
            .parse::<usize>()
            .unwrap()
            - 1;

        let from_stack_content = stack.get_mut(from_stack).unwrap();
        let binding = from_stack_content
            .to_owned();
        let (new_from_stack, items) = binding
            .split_at(from_stack_content.len() - n_to_move);
        stack[from_stack] = Vec::from(new_from_stack);
        stack.get_mut(to_stack).unwrap().append(&mut Vec::from(items));

        stack
    }

    #[test]
    fn test_move_stack_batch() {
        let expect = Vec::from([
            Vec::from([
                String::from("Z"),
                String::from("N"),
                String::from("C"),
                String::from("D"),
            ]),
            Vec::from([String::from("M")]),
            Vec::from([String::from("P")]),
        ]);
        let got = move_stack_batch(
            "move 2 from 2 to 1".to_owned(),
            Vec::from([
                Vec::from([String::from("Z"), String::from("N")]),
                Vec::from([String::from("M"), String::from("C"), String::from("D")]),
                Vec::from([String::from("P")]),
            ]),
        );
        assert_eq!(got, expect);
    }

    pub fn day_5_part_1(reader: BufReader<File>) -> String {
        let mut stack: Stack = Vec::new();

        let mut lines = reader.lines();
        while let Some(line) = lines.next() {
            let l = line.unwrap();
            if l == "" {
                break;
            }

            let sections = parse_line_as_sections(l);
            match sections.first().unwrap().parse::<i32>() {
                // Continue if we get a number as we don't care about that line
                Ok(_) => continue,
                Err(_) => (),
            };

            sections
                .iter()
                .enumerate()
                // .filter(|(_, section)| section.to_owned() != " ")
                .for_each(|(i, section)| {
                    let mut new_section = vec![];
                    if section.to_owned() != " " {
                        new_section.push(section.to_owned());
                    }

                    if let Some(containers) = stack.get_mut(i) {
                        new_section.append(containers);
                    }

                    if stack.len() <= i {
                        stack.push(new_section);
                    } else {
                        stack[i] = new_section;
                    }
                });
        }

        while let Some(line) = lines.next() {
            stack = move_stack(line.unwrap(), stack);
        }

        let res = stack
            .into_iter()
            .filter(|c| c.len() > 0)
            .map(|c| c.last().unwrap().to_owned())
            .fold(String::from(""), |mut s, c| {
                s.push_str(&c);
                s
            });

        res
    }

    #[test]
    fn test_day_5_part_1_example() {
        let expect = "CMZ";
        let got = day_5_part_1(create_reader_from_file(
            "./src/solutions/day_5/example.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_5_part_1_input() {
        let expect = "VPCDMSLWJ";
        let got = day_5_part_1(create_reader_from_file("./src/solutions/day_5/input.txt".to_owned()));
        assert_eq!(got, expect);
    }

    pub fn day_5_part_2(reader: BufReader<File>) -> String {
        let mut stack: Stack = Vec::new();

        let mut lines = reader.lines();
        while let Some(line) = lines.next() {
            let l = line.unwrap();
            if l == "" {
                break;
            }

            let sections = parse_line_as_sections(l);
            match sections.first().unwrap().parse::<i32>() {
                // Continue if we get a number as we don't care about that line
                Ok(_) => continue,
                Err(_) => (),
            };

            sections
                .iter()
                .enumerate()
                // .filter(|(_, section)| section.to_owned() != " ")
                .for_each(|(i, section)| {
                    let mut new_section = vec![];
                    if section.to_owned() != " " {
                        new_section.push(section.to_owned());
                    }

                    if let Some(containers) = stack.get_mut(i) {
                        new_section.append(containers);
                    }

                    if stack.len() <= i {
                        stack.push(new_section);
                    } else {
                        stack[i] = new_section;
                    }
                });
        }

        while let Some(line) = lines.next() {
            stack = move_stack_batch(line.unwrap(), stack);
        }

        let res = stack
            .into_iter()
            .filter(|c| c.len() > 0)
            .map(|c| c.last().unwrap().to_owned())
            .fold(String::from(""), |mut s, c| {
                s.push_str(&c);
                s
            });

        res
    }

    #[test]
    fn test_day_5_part_2_example() {
        let expect = "MCD";

        let got = day_5_part_2(create_reader_from_file(
            "./src/solutions/day_5/example.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_5_part_2_input() {
        let expect = "TPWCGNCCG";

        let got = day_5_part_2(create_reader_from_file("./src/solutions/day_5/input.txt".to_owned()));

        assert_eq!(got, expect);
    }
}
