pub mod day_1;
pub mod day_2;

pub fn solve() {
    day_1::part_1::solve(include_str!("day_1/input.txt"));
    day_1::part_2::solve(include_str!("day_1/input.txt"));
    day_2::part_1::solve(include_str!("day_2/input.txt"));
    day_2::part_2::solve(include_str!("day_2/input.txt"));
}
