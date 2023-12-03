use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::reader::reader::create_reader_from_file;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PointType {
    Beacon,
    Sensor,
    Checked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    pub point_type: PointType,
    pub x: i32,
    pub y: i32,
}

impl From<(PointType, &str)> for Point {
    fn from((point_type, value): (PointType, &str)) -> Self {
        let (x, y) = value.split_once(", ").unwrap();
        Self {
            point_type,
            x: x.trim_start_matches("x=").parse().unwrap(),
            y: y.trim_start_matches("y=").parse().unwrap(),
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.y.cmp(&other.y) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => match self.x.cmp(&other.x) {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Greater => Some(Ordering::Greater),
                Ordering::Equal => Some(Ordering::Equal),
            },
        }
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.x.cmp(&other.x) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            },
        }
    }
}

fn get_sensor_and_beacon_from_string(line: String) -> (Point, Point) {
    let (sensor_cords, beacon_cords) = line
        .trim_start_matches("Sensor at ")
        .split_once(": closest beacon is at ")
        .unwrap();

    (
        Point::from((PointType::Sensor, sensor_cords)),
        Point::from((PointType::Beacon, beacon_cords)),
    )
}

fn get_points_from_sensor(sensor: &Point, max_distance: &i32) -> Vec<Point> {
    let mut checked_points: Vec<Point> = vec![];

    let min_x = sensor.x - *max_distance;
    let max_x = sensor.x + *max_distance;
    for x in min_x..=max_x {}

    checked_points
}

#[test]
fn test_get_points_from_sensor() {
    let mut expect = vec![
        Point {
            point_type: PointType::Checked,
            x: 0,
            y: 2,
        },
        Point {
            point_type: PointType::Checked,
            x: 0,
            y: 1,
        },
        Point {
            point_type: PointType::Checked,
            x: 1,
            y: 1,
        },
        Point {
            point_type: PointType::Checked,
            x: 2,
            y: 0,
        },
        Point {
            point_type: PointType::Checked,
            x: 1,
            y: 0,
        },
        Point {
            point_type: PointType::Checked,
            x: 1,
            y: -1,
        },
        Point {
            point_type: PointType::Checked,
            x: 0,
            y: -2,
        },
        Point {
            point_type: PointType::Checked,
            x: 0,
            y: -1,
        },
        Point {
            point_type: PointType::Checked,
            x: -1,
            y: -1,
        },
        Point {
            point_type: PointType::Checked,
            x: -2,
            y: 0,
        },
        Point {
            point_type: PointType::Checked,
            x: -1,
            y: 0,
        },
        Point {
            point_type: PointType::Checked,
            x: -1,
            y: 1,
        },
    ];
    let mut got = get_points_from_sensor(
        &Point {
            point_type: PointType::Sensor,
            x: 0,
            y: 0,
        },
        &2,
    );

    got.sort();
    expect.sort();

    for e in got.iter() {
        println!("{:?}", e);
    }

    assert_eq!(got, expect);
}

fn get_checked_points(sensor: Point, beacon: Point) -> Vec<Point> {
    let max_distance: i32 = (sensor.x.abs_diff(beacon.x) + sensor.y.abs_diff(beacon.y)) as i32;

    get_points_from_sensor(&sensor, &max_distance)
}

pub fn day_15_part_1(reader: BufReader<File>, y_coord: i32) -> usize {
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(get_sensor_and_beacon_from_string)
        .flat_map(|(sensor, beacon)| {
            let mut points = get_checked_points(sensor, beacon);
            points.push(sensor);
            points.push(beacon);
            points
        })
        .filter(|point| point.y == y_coord)
        .inspect(|p| println!("{:?}", p))
        .count()
}

#[test]
fn test_day_15_part_1_example() {
    let expect = 26;
    let got = day_15_part_1(
        create_reader_from_file("./src/solutions/day_15/example.txt".to_owned()),
        10,
    );
    assert_eq!(got, expect);
}

#[test]
fn test_day_15_part_1_input() {
    let expect = 5_335_787;
    let got = day_15_part_1(
        create_reader_from_file("./src/solutions/day_15/input.txt".to_owned()),
        200_0000,
    );
    assert_eq!(got, expect);
}

pub fn day_15_part_2(reader: BufReader<File>) -> usize {
    0
}

#[test]
fn test_day_15_part_2_example() {
    let expect = 56000011;

    let got = day_15_part_2(create_reader_from_file(
        "./src/solutions/day_15/example.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}

#[test]
fn test_day_15_part_2_input() {
    let expect = 13673971349056;

    let got = day_15_part_2(create_reader_from_file(
        "./src/solutions/day_15/input.txt".to_owned(),
    ));

    assert_eq!(got, expect);
}
