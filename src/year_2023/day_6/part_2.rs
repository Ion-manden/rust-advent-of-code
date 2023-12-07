use super::boat_races::{get_race_from_input, BoatRaceConfig};

pub fn solve(input: &str) -> usize {
    let race = get_race_from_input(input);

    (1..race.time)
        .map(|charge_time| BoatRaceConfig { charge_time })
        .filter(|boat| boat.get_distance_for_race_time(&race.time) > race.distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 71503);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 28360140);
    }
}
