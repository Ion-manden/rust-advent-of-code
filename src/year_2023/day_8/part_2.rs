use num::integer::lcm;

use super::navigation::{Direction, Network};

pub fn solve(input: &str) -> usize {
    let network = Network::from(input);

    let current_positions: Vec<String> = network
        .map
        .keys()
        .filter(|head| head.ends_with("A"))
        .map(|head| head.to_owned())
        .collect();
    let is_goal_position = |pos: &String| pos.ends_with("Z");

    let steps_required: Vec<usize> = current_positions
        .into_iter()
        .map(|current_position| {
            let mut current_position = current_position;
            let mut steps_taken = 0;
            for direction in network.directions.iter().cycle() {
                if is_goal_position(&current_position) {
                    break;
                }

                current_position = match direction {
                    Direction::Left => network.map.get(&current_position).unwrap().0.clone(),
                    Direction::Right => network.map.get(&current_position).unwrap().1.clone(),
                };

                steps_taken += 1;
            }

            steps_taken
        })
        .collect();


    steps_required.iter().fold(1, |acc, &x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example3.txt");
        let result = solve(input);

        assert_eq!(result, 6);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 17972669116327);
    }
}
