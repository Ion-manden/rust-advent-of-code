use std::cmp::Ordering;

use itertools::Itertools;

use super::camel_cards::Hand;

pub fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(Hand::from)
        .sorted_by(|a, b| match Ord::cmp(&a.get_stregth(), &b.get_stregth()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let mut i = 0;
                loop {
                    let a_card = a.cards.iter().nth(i).unwrap();
                    let b_card = b.cards.iter().nth(i).unwrap();
                    match Ord::cmp(a_card, b_card) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {
                            i += 1;
                        },
                    }
                }
            }
        })
        .enumerate()
        .map(|(i, hand)| (i + 1) as i32 * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn solve_example() {
        let input = include_str!("./example.txt");
        let result = solve(input);

        assert_eq!(result, 6440);
    }

    #[test]
    fn solve_input() {
        let input = include_str!("./input.txt");
        let result = solve(input);

        assert_eq!(result, 251927063);
    }
}
