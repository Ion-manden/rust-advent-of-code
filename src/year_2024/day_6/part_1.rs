use crate::libs::matrix::{self, Point};

#[derive(Clone, Copy)]
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}
impl GuardDirection {
    fn to_char(self: &Self) -> char {
        match self {
            GuardDirection::Up => '^',
            GuardDirection::Right => '>',
            GuardDirection::Down => 'v',
            GuardDirection::Left => '<',
        }
    }
    fn from_char(char: &char) -> Self {
        match char {
            '^' => GuardDirection::Up,
            '>' => GuardDirection::Right,
            'v' => GuardDirection::Down,
            '<' => GuardDirection::Left,
            _ => panic!("invalid direction"),
        }
    }

    fn turn_right(self: &Self) -> Self {
        match self {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Right => GuardDirection::Down,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
        }
    }
}

fn get_next_guard_position(
    guard_direction: &GuardDirection,
    guard_position: &Point,
) -> Result<Point, String> {
    match guard_direction {
        GuardDirection::Up => Ok(Point {
            value: guard_position.value,
            x_coord: guard_position.x_coord,
            y_coord: guard_position
                .y_coord
                .checked_sub(1)
                .ok_or("Guard out of top of map")?,
        }),
        GuardDirection::Right => Ok(Point {
            value: guard_position.value,
            x_coord: guard_position.x_coord + 1,
            y_coord: guard_position.y_coord,
        }),
        GuardDirection::Down => Ok(Point {
            value: guard_position.value,
            x_coord: guard_position.x_coord,
            y_coord: guard_position.y_coord + 1,
        }),
        GuardDirection::Left => Ok(Point {
            value: guard_position.value,
            x_coord: guard_position
                .x_coord
                .checked_sub(1)
                .ok_or("Guard out of left part of map")?,
            y_coord: guard_position.y_coord,
        }),
    }
}

fn move_guard(mut map: &mut matrix::Matrix) -> Result<(), String> {
    let guard_position = map
        .find_one_of(&vec![
            GuardDirection::Up.to_char(),
            GuardDirection::Right.to_char(),
            GuardDirection::Down.to_char(),
            GuardDirection::Left.to_char(),
        ])
        .unwrap();

    let mut guard_direction = GuardDirection::from_char(&guard_position.value);

    let proposed_next_guard_position = get_next_guard_position(&guard_direction, &guard_position);

    let next_guard_position = match proposed_next_guard_position {
        Ok(pos) => pos,
        Err(err) => {
            println!("setting last dot {:?}", guard_position);
            map.set(&Point {
                value: 'X',
                x_coord: guard_position.x_coord,
                y_coord: guard_position.y_coord,
            })
            .unwrap();
            return Err(String::from(err));
        }
    };

    // Check if guard is out of the map
    let next_position_char = match map.get(next_guard_position.x_coord, next_guard_position.y_coord)
    {
        Some(char) => char,
        None => {
            map.set(&Point {
                value: 'X',
                x_coord: guard_position.x_coord,
                y_coord: guard_position.y_coord,
            })
            .unwrap();
            return Err(String::from("Guard is out of the map"));
        }
    };

    // If the guard hits an obstacle
    if next_position_char == &'#' {
        guard_direction = guard_direction.turn_right();

        map.set(&Point {
            value: guard_direction.to_char(),
            x_coord: guard_position.x_coord,
            y_coord: guard_position.y_coord,
        })
        .unwrap();
        return Ok(());
    }

    // Set the guard in the next posisition
    map.set(&next_guard_position).unwrap();
    // Mark prev posisition as traversed
    map.set(&Point {
        value: 'X',
        x_coord: guard_position.x_coord,
        y_coord: guard_position.y_coord,
    })
    .unwrap();

    Ok(())
}

pub fn solve(input: &str) -> usize {
    let mut map = matrix::Matrix::from(input);
    loop {
        match move_guard(&mut map) {
            Ok(_) => continue,
            Err(err) => {
                println!("{}", err);
                break;
            }
        };
    }
    map.print();

    map.find_all(&'X').iter().count()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 41);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 5516);
    }
}
