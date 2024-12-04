use crate::libs::matrix;

pub fn solve(input: &str) -> i32 {
    let map = matrix::Matrix::from(input);

    let mut matches_found: i32 = 0;

    let starting_points = map.find_all(&'X');
    for starting_point in starting_points {
        let surrounding_points =
            map.get_surrounding_points(starting_point.x_coord, starting_point.y_coord);
        for surrounding_point in surrounding_points {
            if surrounding_point.value != 'M' {
                continue;
            }

            let (x_diff, y_diff) = starting_point.diff(&surrounding_point);

            let mut next_x_coord = surrounding_point.x_coord as i32 + x_diff;
            let mut next_y_coord = surrounding_point.y_coord as i32 + y_diff;
            let next_char = map.get(next_x_coord as usize, next_y_coord as usize);

            if next_char != Some(&'A') {
                continue;
            }

            next_x_coord = next_x_coord + x_diff;
            next_y_coord = next_y_coord + y_diff;
            let next_char = map.get(next_x_coord as usize, next_y_coord as usize);

            if next_char != Some(&'S') {
                continue;
            }

            matches_found += 1;
        }
    }

    matches_found
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 18);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 2639);
    }
}
