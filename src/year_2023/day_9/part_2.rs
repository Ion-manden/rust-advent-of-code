use super::predict_sequence::predict_prev_number;

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(predict_prev_number)
        .sum()
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

        assert_eq!(result, 1053);
    }
}
