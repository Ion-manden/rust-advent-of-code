use super::converstion_map::create_conversions_vec_from_str;

pub fn solve(input: &str) -> usize {
    let conversions = create_conversions_vec_from_str(input);

    let seeds: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    seeds
        .into_iter()
        .map(|mut seed| {
            conversions.iter().for_each(|conversions| {
                seed = conversions
                    .iter()
                    .find(|(from, _to, range)| &seed >= from && &seed < &(from + range))
                    .map_or(seed, |(from, to, _range)| {
                        to + (&seed).abs_diff(from.to_owned())
                    })
                    .to_owned()
            });

            seed
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 35);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 251346198);
    }
}
