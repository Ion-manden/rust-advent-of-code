use super::parse::{GameResult, GameResults};

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(GameResults::from)
        .map(|game| {
            game.games.iter().fold(
                GameResult {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut state, game| {
                    if state.red < game.red {
                        state.red = game.red;
                    }
                    if state.green < game.green {
                        state.green = game.green;
                    }
                    if state.blue < game.blue {
                        state.blue = game.blue;
                    }

                    state
                },
            )
        })
        .map(|game| game.red * game.green * game.blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 2286);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 71220);
    }
}
