use crate::libs::matrix;

pub fn solve(input: &str) -> i32 {
    let map = matrix::Matrix::from(input);

    let mut matches_found: i32 = 0;

    let starting_points = map.find_all(&'A');
    for starting_point in starting_points {
        if starting_point.x_coord == 0 || starting_point.y_coord == 0 {
            continue;
        }

        let top_left = map.get(starting_point.x_coord - 1, starting_point.y_coord - 1);
        let top_right = map.get(starting_point.x_coord + 1, starting_point.y_coord - 1);

        let bottom_left = map.get(starting_point.x_coord - 1, starting_point.y_coord + 1);
        let bottom_right = map.get(starting_point.x_coord + 1, starting_point.y_coord + 1);

        if top_left == None || top_right == None || bottom_left == None || bottom_right == None {
            continue;
        }

        let line_from_top_left: &str = &format!(
            "{}{}{}",
            top_left.unwrap(),
            starting_point.value,
            bottom_right.unwrap()
        );
        let line_from_top_right: &str = &format!(
            "{}{}{}",
            top_right.unwrap(),
            starting_point.value,
            bottom_left.unwrap()
        );

        let options: Vec<&str> = vec!["MAS", "SAM"];

        if options.contains(&line_from_top_left) && options.contains(&line_from_top_right) {
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

        assert_eq!(result, 9);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 2005);
    }
}
