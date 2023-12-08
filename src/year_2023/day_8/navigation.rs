use std::collections::HashMap;

pub struct Network {
    pub directions: Vec<Direction>,
    pub map: HashMap<String, (String, String)>,
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let directions = lines.next().unwrap().chars().map(Direction::from).collect();
        let map = value
            .lines()
            .skip(2)
            .map(|line| -> (String, (String, String)) {
                let (head, paths) = line.split_once(" = ").unwrap();

                let brackets: &[_] = &['(', ')'];
                let (left, right) = paths.trim_matches(brackets).split_once(", ").unwrap();

                (head.to_owned(), (left.to_owned(), right.to_owned()))
            })
            .fold(HashMap::new(), |mut state, (head, paths)| {
                state.insert(head, paths);
                state
            });

        Self { directions, map }
    }
}

pub enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            c => panic!("Unimplemented char {c}"),
        }
    }
}

pub struct Node {
    pub head: String,
    pub left: String,
    pub right: String,
}

impl From<&str> for Node {
    fn from(line: &str) -> Self {
        let (head, paths) = line.split_once(" = ").unwrap();

        let brackets: &[_] = &['(', ')'];
        let (left, right) = paths.trim_matches(brackets).split_once(", ").unwrap();

        Self {
            head: head.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }
}
