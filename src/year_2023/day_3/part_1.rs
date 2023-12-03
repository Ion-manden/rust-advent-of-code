use itertools::Itertools;

use crate::libs::matrix::{Matrix, Point};

pub fn solve(input: &str) -> i32 {
    let symbols = vec!["$", "%", "/", "*", "&", "#", "+", "-", "@", "="];

    let engine_schematics = Matrix::from(input);

    let mut part_numbers_points: Vec<Vec<Point>> = vec![];

    let mut partial_number_points: Vec<Point> = vec![];

    for (y, line) in engine_schematics.values.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if val.parse::<i32>().is_ok() {
                partial_number_points.push(Point {
                    value: val.to_owned(),
                    x_coord: x,
                    y_coord: y,
                })
            } else {
                if !partial_number_points.is_empty() {
                    part_numbers_points.push(partial_number_points.clone());
                    partial_number_points = vec![];
                }
            }
        }
    }

    part_numbers_points
        .into_iter()
        .filter(|part| {
            part.into_iter().any(|point| {
                engine_schematics
                    .get_surrounding_points(point.x_coord, point.y_coord)
                    .iter()
                    .find(|point| symbols.contains(&point.value.as_str()))
                    .is_some()
            })
        })
        .map(|part_number_points| {
            part_number_points
                .into_iter()
                .map(|p| p.value)
                .join("")
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 559667);
    }
}
