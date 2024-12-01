use std::collections::HashMap;

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

    let mut right_location_ids_count: HashMap<i32, u32> = HashMap::new();

    for right_location_id in right_location_ids {
        let current_count_for_id = right_location_ids_count
            .get(&right_location_id)
            .unwrap_or(&0);

        right_location_ids_count.insert(right_location_id, current_count_for_id + 1);
    }

    let mut similarity_score: u32 = 0;

    for left_location_id in left_location_ids {
        let count_from_right = right_location_ids_count
            .get(&left_location_id)
            .unwrap_or(&0);

        let score: u32 = left_location_id as u32 * count_from_right;

        similarity_score += score;
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 31);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 18934359);
    }
}
