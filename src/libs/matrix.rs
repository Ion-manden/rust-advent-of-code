use std::ops::Add;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub values: Vec<Vec<char>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub value: char,
    pub x_coord: usize,
    pub y_coord: usize,
}

impl From<&str> for Matrix {
    fn from(value: &str) -> Self {
        let values: Vec<Vec<char>> = value
            .lines()
            .map(|line| line.chars().map(|c| c).collect())
            .collect();

        Self { values }
    }
}

impl Matrix {
    pub fn find(self: &Self, x_coord: usize, y_coord: usize) -> Option<&char> {
        self.values.iter().nth(y_coord)?.iter().nth(x_coord)
    }

    pub fn get_surrounding_points(self: &Self, x_coord: usize, y_coord: usize) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        for y in y_coord.checked_sub(1).unwrap_or(0)..=y_coord.add(1) {
            for x in x_coord.checked_sub(1).unwrap_or(0)..=x_coord.add(1) {
                if y == y_coord && x == x_coord {
                    continue;
                }

                let point_val = self.find(x, y);
                if point_val.is_some() {
                    points.push(Point {
                        value: point_val.unwrap().to_owned(),
                        x_coord: x,
                        y_coord: y,
                    });
                }
            }
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use super::{Matrix, Point};

    #[test]
    fn test_matrix_values_from_str() {
        let input = r#"vs1
stp
daw"#;
        let matrix = Matrix::from(input);

        assert_eq!(
            matrix.values,
            vec![
                vec!['v', 's', '1'],
                vec!['s', 't', 'p'],
                vec!['d', 'a', 'w']
            ]
        );
    }

    #[test]
    fn test_get_surrounding_points() {
        let input = r#"vs1
stp
daw"#;
        let matrix = Matrix::from(input);
        let mut points = matrix.get_surrounding_points(1, 1);
        points.sort();

        let mut expected = vec![
            Point {
                value: 'v',
                x_coord: 0,
                y_coord: 0,
            },
            Point {
                value: 's',
                x_coord: 1,
                y_coord: 0,
            },
            Point {
                value: '1',
                x_coord: 2,
                y_coord: 0,
            },
            Point {
                value: 's',
                x_coord: 0,
                y_coord: 1,
            },
            Point {
                value: 'p',
                x_coord: 2,
                y_coord: 1,
            },
            Point {
                value: 'd',
                x_coord: 0,
                y_coord: 2,
            },
            Point {
                value: 'a',
                x_coord: 1,
                y_coord: 2,
            },
            Point {
                value: 'w',
                x_coord: 2,
                y_coord: 2,
            },
        ];

        expected.sort();

        assert_eq!(points,expected);
    }
}
