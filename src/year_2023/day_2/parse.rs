use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct GameResult {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}
impl From<&str> for GameResult {
    fn from(value: &str) -> Self {
        value.trim().split(", ").fold(
            GameResult {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut state, part| {
                let (count_part, color) = part.trim().split_once(" ").unwrap();
                let count = count_part.parse::<i32>().unwrap();

                match color {
                    "red" => state.red = count,
                    "green" => state.green = count,
                    "blue" => state.blue = count,
                    c => panic!("Color not found: {c}"),
                };

                state
            },
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct GameResults {
    pub id: i32,
    pub games: Vec<GameResult>,
}

impl From<&str> for GameResults {
    fn from(line: &str) -> Self {
        let (game_id_part, games_part) = line.split_once(": ").unwrap();

        let re = Regex::new(r"Game ([0-9]+)$").unwrap();
        let game_id = re
            .captures(game_id_part)
            .unwrap()
            .get(1)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .unwrap();

        let games: Vec<GameResult> = games_part.split("; ").map(GameResult::from).collect();

        GameResults { id: game_id, games }
    }
}

#[cfg(test)]
mod tests {
    use super::{GameResult, GameResults};

    #[test]
    fn test_parse_game_line() {
        assert_eq!(
            GameResults::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green "),
            GameResults {
                id: 1,
                games: vec![
                    GameResult {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    GameResult {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    GameResult {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            }
        );
    }
}
