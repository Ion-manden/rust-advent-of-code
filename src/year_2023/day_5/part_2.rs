use itertools::Itertools;

use super::converstion_map::create_conversions_vec_from_str;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::{ops::Range, time::Duration};

pub fn solve(input: &str) -> usize {
    let conversions = create_conversions_vec_from_str(input);

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {per_sec}")
            .unwrap(),
    );

    input
        .lines()
        // take first
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .tuples::<(usize, usize)>()
        .inspect(|(start, length)| println!("{start} - {length}"))
        .flat_map(|(start, length)| {
            let range = start..start + length;

            let mut current_seeds: Vec<Range<usize>> = vec![range];

            conversions.iter().for_each(|conversions| {
                let mut conversion_results: Vec<Range<usize>> = vec![];

                let current = current_seeds.clone();

                for next_range in current {
                    let mut offset = 0;

                    loop {
                        let mut range = next_range.clone().skip(offset);
                        let current_seed = match range.next() {
                            Some(seed) => seed,
                            None => {
                                break;
                            }
                        };

                        let conversion = conversions.iter().find(|(from, _to, length)| {
                            &current_seed >= &from && current_seed < (from + length)
                        });

                        match conversion {
                            Some((from, to, length)) => {
                                let upper: usize =
                                    (from + length).min(current_seed + range.len() + 1);

                                let possible_range: usize = upper - current_seed;

                                let new_val = to + (&current_seed).abs_diff(from.to_owned());
                                let range_upper = new_val + possible_range;

                                conversion_results.push(new_val..range_upper);

                                offset += possible_range;
                            }
                            None => {
                                conversion_results.push(current_seed..current_seed + 1);
                                offset += 1;
                            }
                        }
                    }
                }

                current_seeds = conversion_results;
            });

            current_seeds
        })
        .map(|range| range.start)
        .progress_with(pb)
        .min()
        .unwrap()

    // slower version
    // input
    //     .lines()
    //     .next()
    //     .unwrap()
    //     .split_once(": ")
    //     .unwrap()
    //     .1
    //     .split(" ")
    //     .map(|n| n.parse::<usize>().unwrap())
    //     .tuples::<(usize, usize)>()
    //     .flat_map(|(start, range)| start..(start + range))
    //     .progress_with(pb)
    //     .map(|mut seed| {
    //         conversions.iter().for_each(|conversions| {
    //             seed = conversions
    //                 .iter()
    //                 .find(|(from, _to, range)| &seed >= from && &seed < &(from + range))
    //                 .map_or(seed, |(from, to, _range)| {
    //                     to + (&seed).abs_diff(from.to_owned())
    //                 })
    //                 .to_owned()
    //         });

    //         seed
    //     })
    //     .min()
    //     .unwrap()
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

    #[ignore]
    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 72263011);
    }
}
