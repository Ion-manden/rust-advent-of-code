use itertools::Itertools;

use super::converstion_map::create_conversions_vec_from_str;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::time::Duration;

pub fn solve(input: &str) -> usize {
    let conversions = create_conversions_vec_from_str(input);

    let total: u64 = input
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .tuples::<(usize, usize)>()
        .map(|(_start, range)| range as u64)
        .sum();

    let pb = ProgressBar::new(total);
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}% {pos:>7}/{len:7} {per_sec} eta: {eta_precise}")
            .unwrap()
            .progress_chars("##-"),
    );

    input
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .tuples::<(usize, usize)>()
        .flat_map(|(start, range)| start..(start + range))
        .progress_with(pb)
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

        assert_eq!(result, 46);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 72263011);
    }
}
