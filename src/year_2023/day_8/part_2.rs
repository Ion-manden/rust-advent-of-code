use super::navigation::{Direction, Network};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn solve(input: &str) -> i32 {
    let network = Network::from(input);

    let mut current_positions: Vec<String> = network
        .map
        .keys()
        .filter(|head| head.ends_with("A"))
        .map(|head| head.to_owned())
        .collect();
    let is_goal_position = |pos: &String| pos.ends_with("Z");

    let mut steps_taken = 0;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {per_sec}")
            .unwrap(),
    );

    for direction in network.directions.iter().cycle() {
        pb.inc(1);
        if current_positions.iter().all(is_goal_position) {
            break;
        }

        current_positions = current_positions
            .into_iter()
            .map(|current_position| match direction {
                Direction::Left => network.map.get(&current_position).unwrap().0.clone(),
                Direction::Right => network.map.get(&current_position).unwrap().1.clone(),
            })
            .collect();
        steps_taken += 1;
    }

    steps_taken
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

        assert_eq!(result, 5329815);
    }
}
