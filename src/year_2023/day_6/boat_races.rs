use itertools::Itertools;

pub struct BoatRaceConfig {
    pub charge_time: usize,
}

impl BoatRaceConfig {
    pub fn get_distance_for_race_time(self: &Self, time: &usize) -> usize {
        let time_left: usize = time - self.charge_time;

        let distance_traveled = time_left * self.charge_time;

        distance_traveled
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Race {
    pub time: usize,
    pub distance: usize,
}

pub fn get_races_from_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let mut times = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|time| time.parse::<usize>().unwrap());

    let mut distances = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|time| time.parse::<usize>().unwrap());

    let mut races: Vec<Race> = vec![];

    for _ in 0..times.clone().count() {
        races.push(Race {
            time: times.next().unwrap(),
            distance: distances.next().unwrap(),
        })
    }

    races
}

pub fn get_race_from_input(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .join("")
        .parse::<usize>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .join("")
        .parse::<usize>()
        .unwrap();

    Race{ time, distance }
}

#[cfg(test)]
mod tests {
    use super::get_races_from_input;
    use super::Race;

    #[test]
    fn test_get_races_from_input() {
        let input = include_str!("./example.txt");
        let result = get_races_from_input(input);

        assert_eq!(
            result,
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                },
            ]
        );
    }
}
