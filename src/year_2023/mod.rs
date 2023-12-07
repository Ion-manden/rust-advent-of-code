pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;

pub fn solve() {
    day_1::part_1::solve(include_str!("day_1/input.txt"));
    day_1::part_2::solve(include_str!("day_1/input.txt"));
    day_2::part_1::solve(include_str!("day_2/input.txt"));
    day_2::part_2::solve(include_str!("day_2/input.txt"));
    day_3::part_1::solve(include_str!("day_3/input.txt"));
    day_3::part_2::solve(include_str!("day_3/input.txt"));
    day_4::part_1::solve(include_str!("day_4/input.txt"));
    day_4::part_2::solve(include_str!("day_4/input.txt"));
    day_5::part_1::solve(include_str!("day_5/input.txt"));
    day_5::part_2::solve(include_str!("day_5/input.txt"));
    day_6::part_1::solve(include_str!("day_6/input.txt"));
    day_6::part_2::solve(include_str!("day_6/input.txt"));
}
