use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ord::cmp(&self.get_value(), &other.get_value())
    }
}

impl Card {
    pub fn get_value(self: &Self) -> usize {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::Joker => 1,
        }
    }
    pub fn from_with_joker(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            'J' => Self::Joker,
            c => panic!("Unhandled chard {c}"),
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            c => panic!("Unhandled chard {c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: i32,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (card_part, bid_part) = value.split_once(" ").unwrap();
        let cards = card_part.chars().map(|c| Card::from(c)).collect();
        let bid = bid_part.parse().unwrap();

        Self { cards, bid }
    }
}

impl Hand {
    pub fn from_with_joker(value: &str) -> Self {
        let (card_part, bid_part) = value.split_once(" ").unwrap();
        let cards = card_part
            .chars()
            .map(|c| Card::from_with_joker(c))
            .collect();
        let bid = bid_part.parse().unwrap();

        Self { cards, bid }
    }

    pub fn get_stregth(self: &Self) -> i32 {
        // Five of a kind
        if self.cards.iter().all_equal() {
            return 7;
        }

        // Four of a kind
        let sorted = self.cards.iter().sorted();
        if sorted.clone().skip(1).all_equal() || sorted.clone().take(4).all_equal() {
            return 6;
        }

        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        let card_counts = sorted.clone().counts_by(|c| c);
        let card_count_values = card_counts.values();
        if card_count_values
            .clone()
            .filter(|count| count == &&3)
            .count()
            == 1
            && card_count_values
                .clone()
                .filter(|count| count == &&2)
                .count()
                == 1
        {
            return 5;
        }

        // Three of a kind
        if card_count_values
            .clone()
            .filter(|count| count == &&3)
            .count()
            == 1
        {
            return 4;
        }

        // Two pair
        if card_count_values
            .clone()
            .filter(|count| count == &&2)
            .count()
            == 2
        {
            return 3;
        }

        // One pair
        if card_count_values
            .clone()
            .filter(|count| count == &&2)
            .count()
            == 1
        {
            return 2;
        }

        // High card
        1
    }
}

#[cfg(test)]
mod tests {
    use super::Hand;

    #[test]
    fn test_get_stregth() {
        // Five of a kind
        assert_eq!(Hand::from("KKKKK 22").get_stregth(), 7);
        assert_eq!(Hand::from_with_joker("TJJTJ 7748").get_stregth(), 7);
        // Four of a kind
        assert_eq!(Hand::from("A4444 345").get_stregth(), 6);
        assert_eq!(Hand::from_with_joker("T55J5 684").get_stregth(), 6);
        assert_eq!(Hand::from_with_joker("QQQJA 483").get_stregth(), 6);
        assert_eq!(Hand::from_with_joker("KTJJT 220").get_stregth(), 6);
        // Full house
        assert_eq!(Hand::from("TJJTJ 7748").get_stregth(), 5);
        // Three of a kind
        assert_eq!(Hand::from("T55J5 684").get_stregth(), 4);
        assert_eq!(Hand::from("QQQJA 483").get_stregth(), 4);
        // Two pair
        assert_eq!(Hand::from("KK677 28").get_stregth(), 3);
        assert_eq!(Hand::from("KTJJT 220").get_stregth(), 3);
        // One pair
        assert_eq!(Hand::from("32T3K 765").get_stregth(), 2);
        // High card
        assert_eq!(Hand::from("23456 3").get_stregth(), 1);
    }
}
