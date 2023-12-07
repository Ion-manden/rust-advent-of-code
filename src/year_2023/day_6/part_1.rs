use super::boat_races::{get_races_from_input, BoatRaceConfig, Race};

pub fn solve(input: &str) -> usize {
    let races = get_races_from_input(input);

    races
        .into_iter()
        .map(|race| -> (Race, Vec<BoatRaceConfig>) {
            let mut boats: Vec<BoatRaceConfig> = vec![];

            for i in 1..race.time {
                boats.push(BoatRaceConfig { charge_time: i })
            }

            (race, boats)
        })
        .map(|(race, boats)| -> (Race, Vec<BoatRaceConfig>) {
            let fast_boats: Vec<BoatRaceConfig> = boats
                .into_iter()
                .filter(|boat| boat.get_distance_for_race_time(&race.time) > race.distance)
                .collect();

            (race, fast_boats)
        })
        .map(|(_race, fast_boats)| fast_boats.len())
        .product()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 288);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 449550);
    }
}
