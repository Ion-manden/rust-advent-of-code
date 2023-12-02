use super::parse;
use parse::GameResults;

pub fn solve(input: &str) -> i32 {
    let red_cubes_available = 12;
    let green_cubes_available = 13;
    let blue_cubes_available = 14;

    input
        .lines()
        .map(GameResults::from)
        .filter(|game| {
            game.games.iter().all(|result| {
                result.red <= red_cubes_available
                    && result.green <= green_cubes_available
                    && result.blue <= blue_cubes_available
            })
        })
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 2377);
    }
}
