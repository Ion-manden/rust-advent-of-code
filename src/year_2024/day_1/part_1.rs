pub fn solve(input: &str) -> u32 {
    let mut left_location_ids: Vec<i32> = vec![];
    let mut right_location_ids: Vec<i32> = vec![];


    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();

        let left_location_id: i32 = left.parse().unwrap();
        let right_location_id: i32 = right.parse().unwrap();

        left_location_ids.push(left_location_id);
        right_location_ids.push(right_location_id);
    }
    left_location_ids.sort();
    right_location_ids.sort();


    let mut total_distance: u32 = 0;

    for i in 0..left_location_ids.len(){
        let left_location_id = left_location_ids.get(i).unwrap();
        let right_location_id = right_location_ids.get(i).unwrap();

        total_distance += left_location_id.abs_diff(right_location_id.clone())

    }


    total_distance
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 11);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 2378066);
    }
}