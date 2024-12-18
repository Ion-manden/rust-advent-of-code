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

impl Point {
    // Returns the position difference between root and provided point.
    // Returned value is (difference on x cord, difference on y cord)
    pub fn diff(self: &Self, point: &Point) -> (i32, i32) {
        (
            point.x_coord as i32 - self.x_coord as i32,
            point.y_coord as i32 - self.y_coord as i32,
        )
    }
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
    pub fn print(self: &Self) {
        for line in self.values.clone() {
            let line_string = line.iter().fold(String::from(""), |mut line, c| {
                line.push(*c);
                return line;
            });
            println!("{}", line_string);
        }
    }

    pub fn get_width(self: &Self) -> usize {
        self.values.first().unwrap().len()
    }
    pub fn get_height(self: &Self) -> usize {
        self.values.len()
    }

    pub fn get(self: &Self, x_coord: usize, y_coord: usize) -> Option<&char> {
        self.values.iter().nth(y_coord)?.iter().nth(x_coord)
    }

    pub fn set(&mut self, point: &Point) -> Result<(), String> {
        let current_value = self
            .values
            .get_mut(point.y_coord)
            .ok_or("out of bounds")?
            .get_mut(point.x_coord)
            .ok_or("out of bounds")?;

        *current_value = point.value;

        Ok(())
    }

    pub fn find(self: &Self, value: &char) -> Option<Point> {
        for (y, line) in self.values.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if char == value {
                    return Some(Point {
                        value: char.to_owned(),
                        x_coord: x,
                        y_coord: y,
                    });
                }
            }
        }

        None
    }

    pub fn find_one_of(self: &Self, values: &Vec<char>) -> Option<Point> {
        for (y, line) in self.values.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if values.contains(char) {
                    return Some(Point {
                        value: char.to_owned(),
                        x_coord: x,
                        y_coord: y,
                    });
                }
            }
        }

        None
    }

    pub fn find_all(self: &Self, value: &char) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        for (y, line) in self.values.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if char == value {
                    points.push(Point {
                        value: char.to_owned(),
                        x_coord: x,
                        y_coord: y,
                    })
                }
            }
        }

        points
    }

    pub fn get_surrounding_points(self: &Self, x_coord: usize, y_coord: usize) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        for y in y_coord.checked_sub(1).unwrap_or(0)..=y_coord.add(1) {
            for x in x_coord.checked_sub(1).unwrap_or(0)..=x_coord.add(1) {
                if y == y_coord && x == x_coord {
                    continue;
                }

                let point_val = self.get(x, y);
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

        assert_eq!(points, expected);
    }
}
