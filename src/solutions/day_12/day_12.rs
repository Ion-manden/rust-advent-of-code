use std::{
    cmp::Reverse,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use priority_queue::PriorityQueue;

use crate::reader::reader::create_reader_from_file;

type HeightMap = Vec<Vec<String>>;
fn create_heigth_map_from_reader(reader: BufReader<File>) -> HeightMap {
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().map(|c| c.to_string().to_owned()).collect_vec())
        .collect_vec()
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
    pub height: String,
}

impl Point {
    pub fn get_point_cords(&self) -> String {
        format!("{}:{}", self.y, self.x)
    }
    pub fn calculate_distance_to_point(&self, point: &Point) -> u64 {
        let y_diff = self.y.abs_diff(point.y) as u64;
        let x_diff = self.x.abs_diff(point.x) as u64;

        return y_diff + x_diff;
    }
}

// Finds first point on map matching provided char
fn find_char_on_map(char: String, map: &HeightMap) -> Option<Point> {
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if height == &char {
                return Some(Point {
                    x,
                    y,
                    height: height.to_owned(),
                });
            }
        }
    }

    None
}

fn get_height_from_char(height_char: &String) -> usize {
    if height_char == "S" {
        return get_height_from_char(&"a".to_owned());
    }
    if height_char == "E" {
        return get_height_from_char(&"z".to_owned());
    }

    let chars: String = "abcdefghijklmnopqrstuvwxyz".to_owned();
    chars
        .find(|c: char| c.to_string() == height_char.clone())
        .unwrap()
}

#[test]
fn test_get_height_from_char() {
    let expect = 5;
    let got = get_height_from_char(&"f".to_owned());
    assert_eq!(got, expect);
}

fn get_posible_points(point: &Point, map: &HeightMap) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    if point.y > 0 {
        let y = point.y - 1;
        let x = point.x;
        let height = &map[y][x];
        let p = Point {
            x,
            y,
            height: height.to_owned(),
        };
        points.push(p);
    }

    if point.y < map.len() - 1 {
        let y = point.y + 1;
        let x = point.x;
        let height = &map[y][x];
        let p = Point {
            x,
            y,
            height: height.to_owned(),
        };
        points.push(p);
    }

    if point.x > 0 {
        let y = point.y;
        let x = point.x - 1;
        let height = &map[y][x];
        let p = Point {
            x,
            y,
            height: height.to_owned(),
        };
        points.push(p);
    }

    if point.x < map[0].len() - 1 {
        let y = point.y;
        let x = point.x + 1;
        let height = &map[y][x];
        let p = Point {
            x,
            y,
            height: height.to_owned(),
        };
        points.push(p);
    }

    points
}

#[derive(Eq, Hash, PartialEq)]
struct QueueItem {
    point: Point,
    path_cost: u64,
    // Points here represented as x:y, so 55:107 (x=55, y=107)
    points_cord_visited: Vec<String>,
}
pub fn day_12_part_1(reader: BufReader<File>) -> usize {
    let map = create_heigth_map_from_reader(reader);
    let start_point = find_char_on_map("S".to_owned(), &map).unwrap();
    let goal_point = find_char_on_map("E".to_owned(), &map).unwrap();
    let mut pq: PriorityQueue<QueueItem, Reverse<u64>> = PriorityQueue::new();

    let qi = QueueItem {
        point: start_point,
        path_cost: 0,
        points_cord_visited: Vec::new(),
    };

    let prio = Reverse(qi.point.calculate_distance_to_point(&goal_point));
    pq.push(qi, prio);

    let mut seen_cords: Vec<String> = Vec::new();

    loop {
        let (item, _) = pq.pop().unwrap();

        // if we have tried the point before, skip it
        if seen_cords.contains(&item.point.get_point_cords()) {
            continue;
        }
        seen_cords.push(item.point.get_point_cords());

        if item.point.height == "E" {
            return item.points_cord_visited.len();
        }

        let posible_points = get_posible_points(&item.point, &map)
            .into_iter()
            .filter(|p| {
                get_height_from_char(&p.height) <= get_height_from_char(&item.point.height) + 1
            })
            .collect_vec();

        for p in posible_points.into_iter() {
            let path_cost = item.path_cost + 1 + p.calculate_distance_to_point(&goal_point);
            let prio = Reverse(path_cost);

            let mut qi = QueueItem {
                point: p,
                path_cost,
                points_cord_visited: Vec::new(),
            };
            qi.points_cord_visited
                .append(item.points_cord_visited.clone().as_mut());
            qi.points_cord_visited.push(item.point.get_point_cords());

            pq.push(qi, prio);
        }
    }
}

#[test]
fn test_day_12_part_1_example() {
    let expect = 31;
    let got = day_12_part_1(create_reader_from_file(
        "./src/solutions/day_12/example.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

#[test]
fn test_day_12_part_1_input() {
    let expect = 352;
    let got = day_12_part_1(create_reader_from_file(
        "./src/solutions/day_12/input.txt".to_owned(),
    ));
    assert_eq!(got, expect);
}

pub fn day_12_part_2(reader: BufReader<File>) -> usize {
    let map = create_heigth_map_from_reader(reader);
    let start_point = find_char_on_map("E".to_owned(), &map).unwrap();
    let mut pq: PriorityQueue<QueueItem, Reverse<u64>> = PriorityQueue::new();

    let qi = QueueItem {
        point: start_point,
        path_cost: 0,
        points_cord_visited: Vec::new(),
    };

    let prio = Reverse(qi.path_cost);
    pq.push(qi, prio);

    let mut seen_cords: Vec<String> = Vec::new();

    loop {
        let (item, _) = pq.pop().unwrap();

        // if we have tried the point before, skip it
        if seen_cords.contains(&item.point.get_point_cords()) {
            continue;
        }
        seen_cords.push(item.point.get_point_cords());

        if item.point.height == "a" {
            return item.points_cord_visited.len();
        }

        let posible_points = get_posible_points(&item.point, &map)
            .into_iter()
            .filter(|p| {
                // We look from the other way as we pathfind from the top down
                get_height_from_char(&item.point.height) - 1 <= get_height_from_char(&p.height)
            })
            .collect_vec();

        for p in posible_points.into_iter() {
            let path_cost = item.path_cost + 1;
            let prio = Reverse(path_cost);

            let mut qi = QueueItem {
                point: p,
                path_cost,
                points_cord_visited: Vec::new(),
            };
            qi.points_cord_visited
                .append(item.points_cord_visited.clone().as_mut());
            qi.points_cord_visited.push(item.point.get_point_cords());

            pq.push(qi, prio);
        }
    }
}

#[test]
fn test_day_12_part_2_example() {
    let expect = 29;

    let got = day_12_part_2(create_reader_from_file(
        "./src/solutions/day_12/example.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}

#[test]
fn test_day_12_part_2_input() {
    let expect = 345;

    let got = day_12_part_2(create_reader_from_file(
        "./src/solutions/day_12/input.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}
