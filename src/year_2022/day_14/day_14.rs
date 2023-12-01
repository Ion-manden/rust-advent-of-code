use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::reader::reader::create_reader_from_file;

#[derive(Clone, Copy)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

struct Map {
    pub sand_start: Point,
    pub total_sand_droped: usize,
    matrix: Vec<Vec<char>>,
}

impl Map {
    pub fn print(&self) {
        print!("{}[2J", 27 as char);
        println!("Sand count: {}", self.total_sand_droped);
        for line in self.matrix.iter() {
            for c in line.iter() {
                print!("{}", c);
            }
            println!();
        }
    }

    fn add_rock_to_matrix(&mut self, point: &Point) {
        if self.matrix.first().unwrap().len() <= point.x {
            for line in self.matrix.iter_mut() {
                for _ in line.len()..=point.x {
                    line.push('.');
                }
            }
        }

        if self.matrix.len() <= point.y {
            for _ in self.matrix.len()..=point.y {
                let line: Vec<char> = (0..self.matrix.first().unwrap().len())
                    .map(|_| '.')
                    .collect();
                self.matrix.push(line)
            }
        }

        let point_ref = self
            .matrix
            .get_mut(point.y)
            .unwrap()
            .get_mut(point.x)
            .unwrap();
        *point_ref = '#';
    }

    pub fn add_rock_structure(&mut self, rock_path: &str) {
        let mut prev_edge: Option<Point> = None;
        for edge in rock_path.split(" -> ").map(|part| Point::from(part)) {
            if prev_edge.is_none() {
                prev_edge = Some(edge);
                continue;
            }

            for point in get_points_between_edges(&prev_edge.unwrap(), &edge) {
                self.add_rock_to_matrix(&point);
            }

            prev_edge = Some(edge);
        }
    }

    pub fn get_char_for_position(&self, point: &Point) -> Option<&char> {
        match self.matrix.get(point.y) {
            Some(line) => line.get(point.x),
            None => None,
        }
    }

    pub fn drop_sand(&mut self) -> Result<Point, &str> {
        if self.get_char_for_position(&self.sand_start).unwrap() == &'O' {
            return Err("No more space");
        }

        let mut sand_posistion: Point = self.sand_start;
        loop {
            // Check position under
            match self.get_char_for_position(&Point {
                x: sand_posistion.x,
                y: sand_posistion.y + 1,
            }) {
                Some(c) if c == &'.' => {
                    sand_posistion.y += 1;
                    continue;
                }
                Some(_) => (),
                None => return Err("Falling to void"),
            };
            match self.get_char_for_position(&Point {
                x: sand_posistion.x - 1,
                y: sand_posistion.y + 1,
            }) {
                Some(c) if c == &'.' => {
                    sand_posistion.x -= 1;
                    sand_posistion.y += 1;
                    continue;
                }
                Some(_) => (),
                None => return Err("Falling to void"),
            }
            match self.get_char_for_position(&Point {
                x: sand_posistion.x + 1,
                y: sand_posistion.y + 1,
            }) {
                Some(c) if c == &'.' => {
                    sand_posistion.x += 1;
                    sand_posistion.y += 1;
                    continue;
                }
                Some(_) => (),
                None => return Err("Falling to void"),
            }

            break;
        }

        let point_ref = self
            .matrix
            .get_mut(sand_posistion.y)
            .unwrap()
            .get_mut(sand_posistion.x)
            .unwrap();
        *point_ref = 'O';

        self.total_sand_droped += 1;

        Ok(sand_posistion)
    }
}

fn get_points_between_edges(from: &Point, to: &Point) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    if from.x == to.x {
        for y in from.y.min(to.y)..=from.y.max(to.y) {
            points.push(Point { x: from.x, y });
        }
    }
    if to.y == to.y {
        for x in from.x.min(to.x)..=from.x.max(to.x) {
            points.push(Point { x, y: from.y });
        }
    }

    points
}

pub fn day_14_part_1(reader: BufReader<File>) -> usize {
    let mut map = Map {
        sand_start: Point { x: 500, y: 0 },
        total_sand_droped: 0,
        matrix: vec![vec![]],
    };

    for line in reader.lines() {
        map.add_rock_structure(line.unwrap().as_str());
    }

    loop {
        if map.drop_sand().is_err() {
            break;
        }
    }

    map.total_sand_droped
}

#[test]
fn test_day_14_part_1_example() {
    let expect = 24;
    let got = day_14_part_1(create_reader_from_file(
        "./src/solutions/day_14/example.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

#[test]
fn test_day_14_part_1_input() {
    let expect = 1072;
    let got = day_14_part_1(create_reader_from_file(
        "./src/solutions/day_14/input.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

pub fn day_14_part_2(reader: BufReader<File>) -> usize {
    let mut map = Map {
        sand_start: Point { x: 500, y: 0 },
        total_sand_droped: 0,
        matrix: vec![vec![]],
    };

    for line in reader.lines() {
        map.add_rock_structure(line.unwrap().as_str());
    }

    // Add floor
    map.add_rock_structure(
        format!(
            "0,{} -> {},{}",
            map.matrix.len() + 1,
            map.matrix.first().unwrap().len() + 500,
            map.matrix.len() + 1,
        )
        .as_str(),
    );

    loop {
        if map.drop_sand().is_err() {
            break;
        }
    }

    map.total_sand_droped
}

#[test]
fn test_day_14_part_2_example() {
    let expect = 93;

    let got = day_14_part_2(create_reader_from_file(
        "./src/solutions/day_14/example.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}

#[test]
fn test_day_14_part_2_input() {
    let expect = 24659;

    let got = day_14_part_2(create_reader_from_file(
        "./src/solutions/day_14/input.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}
