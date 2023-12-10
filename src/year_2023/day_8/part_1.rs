use super::navigation::{Direction, Network};

pub fn solve(input: &str) -> i32 {
    let network = Network::from(input);

    let mut current_position = String::from("AAA");
    let goal_position = String::from("ZZZ");
    let mut steps_taken = 0;
    for direction in network.directions.iter().cycle() {
        if current_position == goal_position {
            break;
        }

        current_position = match direction {
            Direction::Left => network.map.get(&current_position).unwrap().0.clone(),
            Direction::Right => network.map.get(&current_position).unwrap().1.clone(),
        };

        steps_taken += 1;
    }

    steps_taken
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example1.txt");
        let result = solve(input);

        assert_eq!(result, 2);

        let input = include_str!("./example2.txt");
        let result = solve(input);

        assert_eq!(result, 6);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 18673);
    }
}
