use super::parse;

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(|line| parse::get_first_and_last_int(line, true))
        .map(|(first, last)| format!("{}{}", first, last))
        .map(|n| n.parse::<i32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example_part_2.txt");
        let result = solve(input);

        assert_eq!(result, 281);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 54265);
    }
}
