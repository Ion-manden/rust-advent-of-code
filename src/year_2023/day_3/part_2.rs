use itertools::Itertools;

use crate::libs::matrix::{Matrix, Point};

pub fn solve(input: &str) -> i32 {
    let engine_schematics = Matrix::from(input);

    let mut part_numbers_points: Vec<Vec<Point>> = vec![];

    let mut partial_number_points: Vec<Point> = vec![];

    let mut stars: Vec<Point> = vec![];

    for (y, line) in engine_schematics.values.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if val == &'*' {
                stars.push(Point {
                    value: val.to_owned(),
                    x_coord: x,
                    y_coord: y,
                });
            }

            if val.to_string().parse::<i32>().is_ok() {
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

    let gears: Vec<Point> = stars
        .into_iter()
        .filter(|star| {
            let surrounding_points_to_star =
                engine_schematics.get_surrounding_points(star.x_coord, star.y_coord);

            part_numbers_points
                .clone()
                .into_iter()
                .filter(|part| {
                    part.into_iter()
                        .any(|point| surrounding_points_to_star.contains(point))
                })
                .collect::<Vec<Vec<Point>>>()
                .len()
                .eq(&2)
        })
        .collect();

    gears
        .into_iter()
        .map(|gear| {
            part_numbers_points
                .iter()
                .filter(|part| {
                    part.into_iter().any(|point| {
                        engine_schematics
                            .get_surrounding_points(point.x_coord, point.y_coord)
                            .into_iter()
                            .find(|point| &gear == point)
                            .is_some()
                    })
                })
                .map(|part_number_points| {
                    part_number_points
                        .into_iter()
                        .map(|p| p.value.clone())
                        .join("")
                        .parse::<i32>()
                        .unwrap()
                })
                .product::<i32>()
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

        assert_eq!(result, 467835);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 86841457);
    }
}
