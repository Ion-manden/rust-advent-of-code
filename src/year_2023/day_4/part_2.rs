use std::collections::HashMap;

use super::scratchcard::Scratchcard;

pub fn solve(input: &str) -> i32 {
    let mut card_count: i32 = 0;

    let mut card_copies: HashMap<i32, i32> = HashMap::new();

    for card in input
        .lines()
        .map(Scratchcard::from)
        .collect::<Vec<Scratchcard>>()
    {
        // Have to do this because i can't borrow a mutable and immutable ref to the same var
        let card_copies_clone = card_copies.clone();
        let copy_count = card_copies_clone.get(&card.id).unwrap_or(&0);

        // Add one for original
        card_count += 1;
        // Add copies
        card_count += copy_count;

        for i in 1..=card.get_matching_numbers().len() {
            let card_id: i32 = card.id + i as i32;
            let current_card_copies = card_copies.get(&card_id).unwrap_or(&0);

            let new_card_copies: i32 = current_card_copies + copy_count + 1;

            card_copies.insert(card_id, new_card_copies);
        }
    }

    card_count
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 30);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 5329815);
    }
}
