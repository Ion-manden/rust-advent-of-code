use super::report;

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line: &str| -> report::Report {
            line.split(" ")
                .map(|split: &str| split.parse().unwrap())
                .collect()
        })
        .filter(report::is_report_safe)
        .count()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 564);
    }
}
