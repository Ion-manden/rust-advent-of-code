use super::safty_manual;

pub fn solve(input: &str) -> i32 {
    let (rules_section, updates_section) = input.split_once("\n\n").unwrap();

    let page_order_rules: Vec<safty_manual::PageOrderingRule> = rules_section
        .lines()
        .map(safty_manual::PageOrderingRule::from)
        .collect();

    let updates: Vec<safty_manual::SaftyManualUpdates> = updates_section
        .lines()
        .map(safty_manual::SaftyManualUpdates::from)
        .collect();

    updates
        .into_iter()
        .filter(|update| !update.is_valid_update(&page_order_rules))
        .map(|update| safty_manual::create_fixed_update(&update, &page_order_rules))
        .inspect(|update| assert!(update.is_valid_update(&page_order_rules)))
        .map(|update| safty_manual::get_middle_number(&update.page_numbers))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 123);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 6767);
    }
}
