use super::scratchcard::Scratchcard;

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(Scratchcard::from)
        .map(|card| -> i32 {
            card.get_matching_numbers().into_iter().fold(0, |state, _| {
                if state == 0 {
                    return 1;
                } else {
                    return state * 2;
                }
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 13);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 21105);
    }
}
