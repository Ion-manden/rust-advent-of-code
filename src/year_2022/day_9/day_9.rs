use itertools::Itertools;
use std::{
    borrow::BorrowMut,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use crate::reader::reader::create_reader_from_file;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("{}: {}", "Invalid direction", s)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Knot {
    pub x: i32,
    pub y: i32,
}

impl Knot {
    pub fn move_up(&mut self) {
        self.y += 1;
    }
    pub fn move_down(&mut self) {
        self.y -= 1;
    }
    pub fn move_left(&mut self) {
        self.x -= 1;
    }
    pub fn move_right(&mut self) {
        self.x += 1;
    }
}

fn move_head(head: &mut Knot, direction: &Direction) {
    match direction {
        Direction::Up => head.move_up(),
        Direction::Down => head.move_down(),
        Direction::Left => head.move_left(),
        Direction::Right => head.move_right(),
    }
}
#[test]
fn test_move_head() {
    {
        let expect = Knot { x: 2, y: 1 };
        let mut got = Knot { x: 1, y: 1 };
        move_head(&mut got, &Direction::Right);
        assert_eq!(got.x, expect.x);
        assert_eq!(got.y, expect.y);
    }
    {
        let expect = Knot { x: 1, y: 0 };
        let mut got = Knot { x: 1, y: 1 };
        move_head(&mut got, &Direction::Down);
        assert_eq!(got.x, expect.x);
        assert_eq!(got.y, expect.y);
    }
}

fn move_knot(knot: &mut Knot, knot_in_front: &Knot) {
    // if front know is only one space away, don't do anything
    if (knot.x - knot_in_front.x).abs() < 2 && (knot.y - knot_in_front.y).abs() < 2 {
        return;
    }

    // Check if directly on line with front knot
    if (knot.x - knot_in_front.x).abs() >= 2 && knot.y == knot_in_front.y {
        if knot.x > knot_in_front.x {
            knot.move_left();
            return;
        }
        if knot.x < knot_in_front.x {
            knot.move_right();
            return;
        }
    }

    // Check if directly above or below front knot
    if (knot.y - knot_in_front.y).abs() >= 2 && knot.x == knot_in_front.x {
        if knot.y > knot_in_front.y {
            knot.move_down();
            return;
        }
        if knot.y < knot_in_front.y {
            knot.move_up();
            return;
        }
    }

    if knot.x > knot_in_front.x {
        knot.move_left();
    }
    if knot.x < knot_in_front.x {
        knot.move_right();
    }
    if knot.y > knot_in_front.y {
        knot.move_down();
    }
    if knot.y < knot_in_front.y {
        knot.move_up();
    }
}
#[test]
fn test_move_knot() {
    {
        let expect = Knot { x: 1, y: 1 };
        let mut got = Knot { x: 1, y: 1 };
        move_knot(&mut got, &Knot { x: 1, y: 1 });
        assert_eq!(got.x, expect.x);
        assert_eq!(got.y, expect.y);
    }
    {
        let expect = Knot { x: 2, y: 1 };
        let mut got = Knot { x: 1, y: 1 };
        move_knot(&mut got, &Knot { x: 3, y: 1 });
        assert_eq!(got.x, expect.x);
        assert_eq!(got.y, expect.y);
    }
    {
        let expect = Knot { x: 2, y: 2 };
        let mut got = Knot { x: 1, y: 1 };
        move_knot(&mut got, &Knot { x: 2, y: 3 });
        assert_eq!(got.x, expect.x);
        assert_eq!(got.y, expect.y);
    }
}

pub fn day_9_part_1(reader: BufReader<File>) -> usize {
    let mut robe: Vec<Knot> = Vec::new();
    // Push head
    robe.push(Knot { x: 0, y: 0 });

    // Push knots
    robe.push(Knot { x: 0, y: 0 });

    let mut visited_tiles: Vec<Knot> = Vec::new();
    // add inital tile
    visited_tiles.push(Knot { x: 0, y: 0 });

    for l in reader.lines() {
        let line = l.unwrap();
        let tup = line.split_once(" ").unwrap();
        let direction = Direction::from_str(tup.0).unwrap();
        let distance: i32 = tup.1.parse::<i32>().unwrap();

        for _ in 0..distance {
            move_head(robe.first_mut().unwrap(), &direction);

            for i in 1..robe.len() {
                let pk = robe[i - 1];
                let k = robe[i].borrow_mut();

                move_knot(k, &pk);
            }
            visited_tiles.push(robe.last().unwrap().clone());
        }
    }

    visited_tiles
        .into_iter()
        .map(|k| format!("{}:{}", k.x, k.y))
        .unique()
        .count()
}

#[test]
fn test_day_9_part_1_example() {
    let expect = 13;
    let got = day_9_part_1(create_reader_from_file(
        "./src/solutions/day_9/example.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

#[test]
fn test_day_9_part_1_input() {
    let expect = 6243;
    let got = day_9_part_1(create_reader_from_file(
        "./src/solutions/day_9/input.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

pub fn day_9_part_2(reader: BufReader<File>) -> usize {
    let mut robe: Vec<Knot> = Vec::new();
    // Push head
    robe.push(Knot { x: 0, y: 0 });

    // Push 9 knots
    for _ in 0..9 {
        robe.push(Knot { x: 0, y: 0 });
    }

    let mut visited_tiles: Vec<Knot> = Vec::new();
    // add inital tile
    visited_tiles.push(Knot { x: 0, y: 0 });

    for l in reader.lines() {
        let line = l.unwrap();
        let tup = line.split_once(" ").unwrap();
        let direction = Direction::from_str(tup.0).unwrap();
        let distance: i32 = tup.1.parse::<i32>().unwrap();

        for _ in 0..distance {
            move_head(robe.first_mut().unwrap(), &direction);

            for i in 1..robe.len() {
                let pk = robe[i - 1];
                let k = robe[i].borrow_mut();

                move_knot(k, &pk);
            }
            visited_tiles.push(robe.last().unwrap().clone());
        }
    }

    visited_tiles
        .into_iter()
        .map(|k| format!("{}:{}", k.x, k.y))
        .unique()
        .count()
}

#[test]
fn test_day_9_part_2_example() {
    let expect = 1;

    let got = day_9_part_2(create_reader_from_file(
        "./src/solutions/day_9/example.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}

#[test]
fn test_day_9_part_2_input() {
    let expect = 2630;

    let got = day_9_part_2(create_reader_from_file(
        "./src/solutions/day_9/input.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}
